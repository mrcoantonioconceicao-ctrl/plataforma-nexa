// ~/nexavor/services/wallet/src/infrastructure/postgres_wallet_repository.rs

use crate::domain::wallet::Wallet;
use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use sqlx::PgPool;
use uuid::Uuid;

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

        // Utilizando query_as sem o macro '!' para evitar checagem de banco em tempo de compilação
        let rec = sqlx::query_as::<_, (Uuid, Uuid, Decimal, String, DateTime<Utc>)>(
            r#"
            SELECT id, user_id, balance, currency, created_at
            FROM wallets WHERE id = $1
            "#,
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await;

        match rec {
            Ok(Some(row)) => Ok(Some(Wallet {
                id: row.0,
                user_id: row.1,
                balance: row.2,
                currency: row.3,
                created_at: row.4,
            })),
            Ok(None) => Ok(None),
            Err(e) => {
                println!(
                    "[Aviso PostgresWalletRepository] Erro ao buscar no banco: {}",
                    e
                );
                Ok(None)
            }
        }
    }
}
