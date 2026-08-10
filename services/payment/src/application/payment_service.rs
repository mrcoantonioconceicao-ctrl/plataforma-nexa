// ~/nexavor/services/payment/src/application/payment_service.rs

use crate::domain::payment::{Payment, PaymentStatus, PaymentType};
use crate::domain::payment_repository::PaymentRepository;
use uuid::Uuid;
use rust_decimal::Decimal;

pub struct PaymentService<R: PaymentRepository> {
    repo: R,
}

impl<R: PaymentRepository> PaymentService<R> {
    pub fn new(repo: R) -> Self {
        Self { repo }
    }

    pub async fn process_payment(
        &self,
        wallet_id: Uuid,
        amount: Decimal,
        currency: String,
        payment_type: PaymentType,
    ) -> Result<Payment, String> {
        let mut payment = Payment::new(wallet_id, amount, currency, payment_type, None);
        self.repo.save(payment.clone());

        payment.complete();
        self.repo.update_status(payment.id, PaymentStatus::Completed);
        
        Ok(payment)
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
    use rust_decimal_macros::dec;

    #[tokio::test]
    async fn test_process_payment_success() {
        let repo = InMemoryPaymentRepository::new();
        let service = PaymentService::new(repo);

        let wallet_id = Uuid::new_v4();
        let result = service
            .process_payment(wallet_id, dec!(100.00), "BRL".to_string(), PaymentType::PixIn)
            .await;

        assert!(result.is_ok());
        let payment = result.unwrap();
        assert_eq!(payment.status, PaymentStatus::Completed);
        assert_eq!(payment.amount, dec!(100.00));
    }

    #[tokio::test]
    async fn test_fail_payment_success() {
        let repo = InMemoryPaymentRepository::new();
        let service = PaymentService::new(repo);

        let wallet_id = Uuid::new_v4();
        let payment = service
            .process_payment(wallet_id, dec!(50.00), "BRL".to_string(), PaymentType::PixOut)
            .await
            .unwrap();

        let fail_result = service.fail_payment(payment.id).await;
        assert!(fail_result.is_ok());

        let updated_payment = service.repo.find_by_id(payment.id).unwrap();
        assert_eq!(updated_payment.status, PaymentStatus::Failed);
    }
}

