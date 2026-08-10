use sqlx::PgPool;
use uuid::Uuid;

use crate::domain::ledger_entry::LedgerEntry;
use crate::infrastructure::ledger_repository::LedgerRepository;

#[derive(Clone)]
pub struct PostgresLedgerRepository {
    pool: PgPool,
}

impl PostgresLedgerRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub fn pool(&self) -> &PgPool {
        &self.pool
    }
}

impl LedgerRepository for PostgresLedgerRepository {
    fn save(&self, entry: LedgerEntry) {
        let pool = self.pool.clone();

        tokio::spawn(async move {
            if let Err(error) = sqlx::query(
                r#"
                INSERT INTO ledger_entries
                (
                    id,
                    journal_id,
                    account_id,
                    currency,
                    debit,
                    credit,
                    created_at
                )
                VALUES
                (
                    $1,$2,$3,$4,$5,$6,$7
                )
                "#,
            )
            .bind(entry.id)
            .bind(entry.journal_id)
            .bind(entry.account_id)
            .bind(entry.currency)
            .bind(entry.debit)
            .bind(entry.credit)
            .bind(entry.created_at)
            .execute(&pool)
            .await
            {
                eprintln!("Erro ao salvar LedgerEntry: {}", error);
            }
        });
    }

    fn find_by_id(&self, _id: Uuid) -> Option<LedgerEntry> {
        // Implementação temporária.
        // Será substituída por consultas SQL reais na próxima etapa.
        None
    }

    fn find_all(&self) -> Vec<LedgerEntry> {
        // Implementação temporária.
        Vec::new()
    }

    fn find_by_account(&self, _account: &str) -> Vec<LedgerEntry> {
        // Implementação temporária.
        Vec::new()
    }
}
