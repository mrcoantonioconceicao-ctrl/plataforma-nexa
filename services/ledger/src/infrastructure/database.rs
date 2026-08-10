// ~/nexavor/services/ledger/src/infrastructure/database.rs

use sqlx::PgPool;
use std::env;
use std::time::Duration;

#[derive(Clone)]
pub struct Database {
    pub pool: PgPool,
}

impl Database {
    pub async fn connect(database_url: &str) -> Result<Self, sqlx::Error> {
        let pool = sqlx::postgres::PgPoolOptions::new()
            .max_connections(20)
            .min_connections(5)
            .acquire_timeout(Duration::from_secs(5))
            .idle_timeout(Duration::from_secs(600))
            .connect(database_url)
            .await?;

        Ok(Self { pool })
    }

    pub async fn connect_from_env() -> Result<Self, sqlx::Error> {
        let database_url = env::var("DATABASE_URL").unwrap_or_else(|_| {
            "postgres://postgres:postgres@localhost:5432/nexavor_ledger".to_string()
        });

        Self::connect(&database_url).await
    }
}
