// ~/nexavor/services/wallet/src/infrastructure/postgres_wallet_repository.rs

use sqlx::PgPool;
use uuid::Uuid;
use crate::domain::wallet::Wallet;
use rust_decimal::Decimal;
use chrono::{DateTime, Utc};

#[derive(Clone)]
pub struct PostgresWalletRepository {
    pool: PgPool,
}

impl PostgresWalletRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn save(&self, wallet: &Wallet) -> Result<(), sqlx::Error> {
        sqlx::query(
            r#"
            INSERT INTO wallets (id, user_id, balance, currency, created_at)
            VALUES ($1, $2, $3, $4, $5)
            ON CONFLICT (id) DO UPDATE SET balance = $3
            "#,
        )
        .bind(wallet.id)
        .bind(wallet.user_id)
        .bind(wallet.balance)
        .bind(&wallet.currency)
        .bind(wallet.created_at)
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    pub async fn find_by_id(&self, id: Uuid) -> Result<Option<Wallet>, sqlx::Error> {
        struct WalletRow {
            id: Uuid,
            user_id: Uuid,
            balance: Decimal,
            currency: String,
            created_at: DateTime<Utc>,
        }

        let row = sqlx::query_as!(
            WalletRow,
            r#"
            SELECT id, user_id, balance, currency, created_at
            FROM wallets WHERE id = $1
            "#,
            id
        )
        .fetch_optional(&self.pool)
        .await?;

        Ok(row.map(|r| Wallet {
            id: r.id,
            user_id: r.user_id,
            balance: r.balance,
            currency: r.currency,
            created_at: r.created_at,
        }))
    }
}

