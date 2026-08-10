use sqlx::{PgPool, postgres::PgPoolOptions};
use std::{env, time::Duration};

pub async fn create_pool() -> Result<PgPool, sqlx::Error> {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL não definida.");

    PgPoolOptions::new()
        .max_connections(50)
        .min_connections(5)
        .acquire_timeout(Duration::from_secs(10))
        .idle_timeout(Duration::from_secs(600))
        .max_lifetime(Duration::from_secs(1800))
        .test_before_acquire(true)
        .connect(&database_url)
        .await
}
