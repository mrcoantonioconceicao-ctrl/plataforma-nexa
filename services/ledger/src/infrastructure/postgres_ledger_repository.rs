// ~/nexavor/services/ledger/src/infrastructure/postgres_ledger_repository.rs

use crate::domain::ledger_entry::LedgerEntry;
use crate::infrastructure::ledger_repository::LedgerRepository;
use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use sqlx::{FromRow, PgPool};
use uuid::Uuid;

#[derive(FromRow)]
struct LedgerEntryRow {
    id: Uuid,
    journal_id: Uuid,
    account_id: Uuid,
    currency: String,
    debit: Decimal,
    credit: Decimal,
    created_at: DateTime<Utc>,
}

impl From<LedgerEntryRow> for LedgerEntry {
    fn from(row: LedgerEntryRow) -> Self {
        Self {
            id: row.id,
            journal_id: row.journal_id,
            account_id: row.account_id,
            currency: row.currency,
            debit: row.debit,
            credit: row.credit,
            created_at: row.created_at,
        }
    }
}

#[derive(Clone)]
pub struct PostgresLedgerRepository {
    pool: PgPool,
    rt: tokio::runtime::Handle,
}

impl PostgresLedgerRepository {
    pub fn new(pool: PgPool) -> Self {
        Self {
            pool,
            rt: tokio::runtime::Handle::current(),
        }
    }
}

impl LedgerRepository for PostgresLedgerRepository {
    fn save(&self, entry: LedgerEntry) {
        let pool = self.pool.clone();
        self.rt.block_on(async move {
            let _ = sqlx::query(
                r#"
                INSERT INTO ledger_entries (id, journal_id, account_id, currency, debit, credit, created_at)
                VALUES ($1, $2, $3, $4, $5, $6, $7)
                ON CONFLICT (id) DO NOTHING
                "#
            )
            .bind(entry.id)
            .bind(entry.journal_id)
            .bind(entry.account_id)
            .bind(entry.currency)
            .bind(entry.debit)
            .bind(entry.credit)
            .bind(entry.created_at)
            .execute(&pool)
            .await;
        });
    }

    fn find_by_id(&self, id: Uuid) -> Option<LedgerEntry> {
        let pool = self.pool.clone();
        self.rt.block_on(async move {
            let row = sqlx::query_as::<_, LedgerEntryRow>(
                "SELECT id, journal_id, account_id, currency, debit, credit, created_at FROM ledger_entries WHERE id = $1"
            )
            .bind(id)
            .fetch_optional(&pool)
            .await
            .ok()?;

            row.map(Into::into)
        })
    }

    fn find_all(&self) -> Vec<LedgerEntry> {
        let pool = self.pool.clone();
        self.rt.block_on(async move {
            let rows = sqlx::query_as::<_, LedgerEntryRow>(
                "SELECT id, journal_id, account_id, currency, debit, credit, created_at FROM ledger_entries ORDER BY created_at DESC"
            )
            .fetch_all(&pool)
            .await
            .unwrap_or_default();

            rows.into_iter().map(Into::into).collect()
        })
    }

    fn find_by_account(&self, account: &str) -> Vec<LedgerEntry> {
        let pool = self.pool.clone();
        let account_uuid = Uuid::parse_str(account).unwrap_or_else(|_| Uuid::nil());
        self.rt.block_on(async move {
            let rows = sqlx::query_as::<_, LedgerEntryRow>(
                "SELECT id, journal_id, account_id, currency, debit, credit, created_at FROM ledger_entries WHERE account_id = $1 ORDER BY created_at DESC"
            )
            .bind(account_uuid)
            .fetch_all(&pool)
            .await
            .unwrap_or_default();

            rows.into_iter().map(Into::into).collect()
        })
    }
}
