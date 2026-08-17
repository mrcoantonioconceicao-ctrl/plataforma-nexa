// ~/nexavor/services/payment/src/main.rs

mod application;
mod domain;
mod infrastructure;

use application::payment_service::PaymentService;
use async_trait::async_trait;
use domain::payment::PaymentType;
use domain::payment_repository::PaymentRepository; // Importando o trait para expor os métodos
use domain::services_port::{LedgerPort, WalletPort};
use infrastructure::in_memory_payment_repository::InMemoryPaymentRepository;
use rust_decimal::Decimal;
use rust_decimal_macros::dec;
use uuid::Uuid;

struct DummyWalletAdapter;
#[async_trait]
impl WalletPort for DummyWalletAdapter {
    async fn debit(&self, _wallet_id: Uuid, _amount: Decimal) -> Result<(), String> {
        Ok(())
    }
    async fn credit(&self, _wallet_id: Uuid, _amount: Decimal) -> Result<(), String> {
        Ok(())
    }
}

struct DummyLedgerAdapter;
#[async_trait]
impl LedgerPort for DummyLedgerAdapter {
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

#[tokio::main]
async fn main() {
    println!("Inicializando Nexavor Payment Engine com EDA (Event-Driven Architecture)...");

    let repo = InMemoryPaymentRepository::new();
    let wallet = DummyWalletAdapter;
    let ledger = DummyLedgerAdapter;
    let service = PaymentService::new(repo, wallet, ledger);

    let wallet_id = Uuid::new_v4();
    let amount = dec!(1000.00);
    let currency = "BRL".to_string();

    // 1. Executando fluxo de sucesso
    match service
        .process_payment(wallet_id, amount, currency, PaymentType::PixIn)
        .await
    {
        Ok((payment, event)) => {
            println!(
                "Transação executada com sucesso! ID: {}, Status: {:?}",
                payment.id, payment.status
            );
            println!("Evento de Domínio emitido: {:?}", event);

            // 2. Demonstrando o uso do fail_payment para testes/rollback
            if let Err(e) = service.fail_payment(payment.id).await {
                println!("Erro ao falhar pagamento: {}", e);
            } else {
                let updated = service.repo.find_by_id(payment.id).unwrap();
                println!(
                    "Status atualizado após falha explícita: ID: {}, Status: {:?}",
                    updated.id, updated.status
                );
            }
        }
        Err(e) => {
            println!("Erro no processamento do evento: {}", e);
        }
    }
}
