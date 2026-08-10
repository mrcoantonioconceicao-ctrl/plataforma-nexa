// ~/nexavor/services/ledger/tests/postgres_ledger_integration_tests.rs

use ledger::{
    domain::ledger_entry::LedgerEntry,
    infrastructure::{
        database::Database, ledger_repository::LedgerRepository,
        postgres_ledger_repository::PostgresLedgerRepository,
    },
};
use rust_decimal_macros::dec;
use uuid::Uuid;

/// Tenta conectar ao PostgreSQL de testes.
/// Se o banco não estiver acessível (ex: sem serviço rodando localmente),
/// retorna None para que o teste seja omitido graciosamente sem falhar o build.
async fn setup_test_db() -> Option<PostgresLedgerRepository> {
    let database_url = std::env::var("DATABASE_URL").unwrap_or_else(|_| {
        "postgres://postgres:postgres@localhost:5432/nexavor_ledger".to_string()
    });

    match Database::connect(&database_url).await {
        Ok(db) => Some(PostgresLedgerRepository::new(db.pool)),
        Err(err) => {
            eprintln!(
                "⚠️ [SKIP] Instância PostgreSQL não detectada para teste de integração ({})",
                err
            );
            None
        }
    }
}

#[tokio::test]
async fn test_should_save_and_find_ledger_entry_by_id() {
    let repo = match setup_test_db().await {
        Some(repo) => repo,
        None => return, // Pula o teste graciosamente se o banco estiver offline
    };

    let journal_id = Uuid::new_v4();
    let account_id = Uuid::new_v4();
    let currency = "BRL".to_string();
    let debit = dec!(150.0000);
    let credit = dec!(0.0000);

    let entry = LedgerEntry::new(journal_id, account_id, currency, debit, credit);
    let entry_id = entry.id;

    // Salva no PostgreSQL
    repo.save(entry.clone());

    // Busca pelo ID
    let found = repo.find_by_id(entry_id);

    assert!(
        found.is_some(),
        "Deveria encontrar o lançamento salvo no banco"
    );
    let retrieved = found.unwrap();
    assert_eq!(retrieved.id, entry_id);
    assert_eq!(retrieved.account_id, account_id);
    assert_eq!(retrieved.debit, debit);
    assert_eq!(retrieved.credit, credit);
}

#[tokio::test]
async fn test_should_find_entries_by_account() {
    let repo = match setup_test_db().await {
        Some(repo) => repo,
        None => return, // Pula o teste graciosamente se o banco estiver offline
    };

    let journal_id = Uuid::new_v4();
    let account_id = Uuid::new_v4();
    let currency = "USD".to_string();
    let debit = dec!(0.0000);
    let credit = dec!(500.5000);

    let entry = LedgerEntry::new(journal_id, account_id, currency, debit, credit);
    repo.save(entry.clone());

    let entries = repo.find_by_account(&account_id.to_string());

    assert!(
        !entries.is_empty(),
        "Deveria retornar lançamentos para a conta informada"
    );
    let matched = entries.iter().find(|e| e.id == entry.id);
    assert!(
        matched.is_some(),
        "O lançamento recém-criado deve estar na lista da conta"
    );
}
