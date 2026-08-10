// ~/nexavor/services/payment/src/main.rs

mod application;
mod domain;
mod infrastructure;

use application::payment_service::PaymentService;
use domain::payment::PaymentType;
use infrastructure::in_memory_payment_repository::InMemoryPaymentRepository;
use rust_decimal_macros::dec;
use uuid::Uuid;

#[tokio::main]
async fn main() {
    println!("Inicializando Nexavor Payment Engine...");

    let repo = InMemoryPaymentRepository::new();
    let service = PaymentService::new(repo);

    let wallet_id = Uuid::new_v4();
    let amount = dec!(150.75);
    let currency = "BRL".to_string();

    match service
        .process_payment(wallet_id, amount, currency, PaymentType::PixIn)
        .await
    {
        Ok(payment) => {
            println!(
                "Pagamento processado com sucesso! ID: {}, Status: {:?}",
                payment.id, payment.status
            );
        }
        Err(e) => {
            println!("Erro ao processar pagamento: {}", e);
        }
    }
}
