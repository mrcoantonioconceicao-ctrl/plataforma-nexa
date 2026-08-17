// ~/nexavor/services/payment/src/application/payment_service.rs

use crate::domain::events::PaymentDomainEvent;
use crate::domain::payment::{Payment, PaymentStatus, PaymentType};
use crate::domain::payment_repository::PaymentRepository;
use crate::domain::services_port::{LedgerPort, WalletPort};
use chrono::Utc;
use rust_decimal::Decimal;
use uuid::Uuid;

pub struct PaymentService<R: PaymentRepository, W: WalletPort, L: LedgerPort> {
    pub repo: R,
    wallet_port: W,
    ledger_port: L,
}

impl<R: PaymentRepository, W: WalletPort, L: LedgerPort> PaymentService<R, W, L> {
    pub fn new(repo: R, wallet_port: W, ledger_port: L) -> Self {
        Self {
            repo,
            wallet_port,
            ledger_port,
        }
    }

    pub async fn process_payment(
        &self,
        wallet_id: Uuid,
        amount: Decimal,
        currency: String,
        payment_type: PaymentType,
    ) -> Result<(Payment, PaymentDomainEvent), String> {
        let mut payment = Payment::new(
            wallet_id,
            amount,
            currency.clone(),
            payment_type.clone(),
            None,
        );
        self.repo.save(payment.clone());

        // Evento de Iniciação (BPMN: Task - Iniciar Transação)
        let _init_event = PaymentDomainEvent::PaymentInitiated {
            payment_id: payment.id,
            wallet_id,
            amount,
            currency: currency.clone(),
            timestamp: Utc::now(),
        };

        match payment_type {
            PaymentType::PixIn | PaymentType::StablecoinDeposit => {
                if let Err(e) = self.wallet_port.credit(wallet_id, amount).await {
                    payment.fail();
                    self.repo.update_status(payment.id, PaymentStatus::Failed);
                    let fail_event = PaymentDomainEvent::PaymentFailed {
                        payment_id: payment.id,
                        wallet_id,
                        reason: format!("Wallet credit failed: {}", e),
                        timestamp: Utc::now(),
                    };
                    return Err(format!("Payment failed: {:?}", fail_event));
                }
            }
            PaymentType::PixOut | PaymentType::StablecoinWithdraw => {
                if let Err(e) = self.wallet_port.debit(wallet_id, amount).await {
                    payment.fail();
                    self.repo.update_status(payment.id, PaymentStatus::Failed);
                    let fail_event = PaymentDomainEvent::PaymentFailed {
                        payment_id: payment.id,
                        wallet_id,
                        reason: format!("Wallet debit failed: {}", e),
                        timestamp: Utc::now(),
                    };
                    return Err(format!("Payment failed: {:?}", fail_event));
                }
            }
        }

        let dummy_system_account = Uuid::new_v4();
        let (src, dest) = match payment_type {
            PaymentType::PixIn | PaymentType::StablecoinDeposit => {
                (dummy_system_account, wallet_id)
            }
            PaymentType::PixOut | PaymentType::StablecoinWithdraw => {
                (wallet_id, dummy_system_account)
            }
        };

        if let Err(e) = self
            .ledger_port
            .record_transaction(payment.id, src, dest, amount, currency.clone())
            .await
        {
            payment.fail();
            self.repo.update_status(payment.id, PaymentStatus::Failed);
            let fail_event = PaymentDomainEvent::PaymentFailed {
                payment_id: payment.id,
                wallet_id,
                reason: format!("Ledger recording failed: {}", e),
                timestamp: Utc::now(),
            };
            return Err(format!("Payment failed: {:?}", fail_event));
        }

        payment.complete();
        self.repo
            .update_status(payment.id, PaymentStatus::Completed);

        // Evento de Conclusão (BPMN: End Event - Transação Concluída com Sucesso)
        let completion_event = PaymentDomainEvent::PaymentCompleted {
            payment_id: payment.id,
            wallet_id,
            amount,
            currency,
            timestamp: Utc::now(),
        };

        Ok((payment, completion_event))
    }

    pub async fn fail_payment(&self, payment_id: Uuid) -> Result<(), String> {
        if let Some(mut payment) = self.repo.find_by_id(payment_id) {
            payment.fail();
            self.repo.update_status(payment.id, PaymentStatus::Failed);
            Ok(())
        } else {
            Err("Payment not found".to_string())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::infrastructure::in_memory_payment_repository::InMemoryPaymentRepository;
    use async_trait::async_trait;
    use rust_decimal_macros::dec;

    struct MockWalletAdapter;
    #[async_trait]
    impl WalletPort for MockWalletAdapter {
        async fn debit(&self, _wallet_id: Uuid, _amount: Decimal) -> Result<(), String> {
            Ok(())
        }
        async fn credit(&self, _wallet_id: Uuid, _amount: Decimal) -> Result<(), String> {
            Ok(())
        }
    }

    struct MockLedgerAdapter;
    #[async_trait]
    impl LedgerPort for MockLedgerAdapter {
        async fn record_transaction(
            &self,
            _tx_id: Uuid,
            _src: Uuid,
            _dest: Uuid,
            _amount: Decimal,
            _currency: String,
        ) -> Result<(), String> {
            Ok(())
        }
    }

    #[tokio::test]
    async fn test_process_payment_emits_event() {
        let repo = InMemoryPaymentRepository::new();
        let service = PaymentService::new(repo, MockWalletAdapter, MockLedgerAdapter);

        let wallet_id = Uuid::new_v4();
        let result = service
            .process_payment(
                wallet_id,
                dec!(300.00),
                "BRL".to_string(),
                PaymentType::PixIn,
            )
            .await;

        assert!(result.is_ok());
        let (payment, event) = result.unwrap();
        assert_eq!(payment.status, PaymentStatus::Completed);

        match event {
            PaymentDomainEvent::PaymentCompleted {
                payment_id, amount, ..
            } => {
                assert_eq!(payment_id, payment.id);
                assert_eq!(amount, dec!(300.00));
            }
            _ => panic!("Expected PaymentCompleted event"),
        }
    }

    #[tokio::test]
    async fn test_fail_payment_explicitly() {
        let repo = InMemoryPaymentRepository::new();
        let service = PaymentService::new(repo, MockWalletAdapter, MockLedgerAdapter);

        let wallet_id = Uuid::new_v4();
        let payment = Payment::new(
            wallet_id,
            dec!(150.00),
            "BRL".to_string(),
            PaymentType::PixIn,
            None,
        );
        service.repo.save(payment.clone());

        let result = service.fail_payment(payment.id).await;
        assert!(result.is_ok());

        let found = service.repo.find_by_id(payment.id).unwrap();
        assert_eq!(found.status, PaymentStatus::Failed);
    }
}
