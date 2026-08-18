// ~/nexavor/services/payment/src/main.rs

mod application;
mod domain;
mod infrastructure;

use application::payment_service::PaymentService;
use async_trait::async_trait;
use domain::payment::PaymentType;
use domain::payment_repository::PaymentRepository;
use domain::services_port::{LedgerPort, WalletPort};
use infrastructure::in_memory_payment_repository::InMemoryPaymentRepository;
use rust_decimal::Decimal;
use rust_decimal_macros::dec;
use uuid::Uuid;

// Importando o Journal e JournalEntry corretos do crate ledger
use ledger::domain::JournalEntry;
use ledger::domain::journal::Journal;

struct DummyWalletAdapter;
#[async_trait]
impl WalletPort for DummyWalletAdapter {
    async fn debit(&self, wallet_id: Uuid, amount: Decimal) -> Result<(), String> {
        println!(
            "[WalletPort] Debitando {} da carteira {}",
            amount, wallet_id
        );
        Ok(())
    }
    async fn credit(&self, wallet_id: Uuid, amount: Decimal) -> Result<(), String> {
        println!(
            "[WalletPort] Kreditando {} na carteira {}",
            amount, wallet_id
        );
        Ok(())
    }
}

// Adaptador Real utilizando o Motor Contábil do crate `ledger`
struct RealLedgerAdapter;
#[async_trait]
impl LedgerPort for RealLedgerAdapter {
    async fn record_transaction(
        &self,
        tx_id: Uuid,
        src: Uuid,
        dest: Uuid,
        amount: Decimal,
        currency: String,
    ) -> Result<(), String> {
        println!("[LedgerPort] Processando partidas dobradas via Motor Contábil (Crate Ledger)...");

        // Instanciando as entradas contábeis com os campos corretos da struct JournalEntry do ledger
        let debit_entry = JournalEntry {
            account_id: src,
            currency: currency.clone(),
            debit: amount,
            credit: dec!(0),
        };

        let credit_entry = JournalEntry {
            account_id: dest,
            currency,
            debit: dec!(0),
            credit: amount,
        };

        let journal = Journal::new(vec![debit_entry, credit_entry]);

        // Validação estrita de partidas dobradas do motor contábil
        if journal.is_balanced() {
            println!(
                "  -> Journal Contábil ID: {} validado e balanceado com sucesso! (Total: {})",
                tx_id, amount
            );
            Ok(())
        } else {
            Err("Falha contábil: Journal desbalanceado!".to_string())
        }
    }
}

#[tokio::main]
async fn main() {
    println!("Inicializando Nexavor Payment Engine com EDA e Ledger Real...");

    let repo = InMemoryPaymentRepository::new();
    let wallet = DummyWalletAdapter;
    let ledger = RealLedgerAdapter;
    let service = PaymentService::new(repo, wallet, ledger);

    let wallet_id = Uuid::new_v4();
    let amount = dec!(2500.00);
    let currency = "BRL".to_string();

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
