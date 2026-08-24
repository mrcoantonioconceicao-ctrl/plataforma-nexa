// ~/nexavor/services/wallet/src/main.rs

mod api;
mod domain;
mod infrastructure;

use api::wallet_handler::{AppState, create_wallet, debit_wallet, get_wallet};
use axum::{
    Router,
    routing::{get, post},
};
use infrastructure::postgres_wallet_repository::PostgresWalletRepository;
use sqlx::PgPool;

#[tokio::main]
async fn main() {
    println!("Inicializando Nexavor Wallet Service API com PostgreSQL...");

    let database_url = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgres://postgres:postgres@localhost:5432/nexavor".to_string());

    let pool = match PgPool::connect(&database_url).await {
        Ok(p) => {
            println!("Conexão com o PostgreSQL estabelecida com sucesso!");
            p
        }
        Err(e) => {
            eprintln!(
                "Aviso: Falha na conexão com o banco ({}), subindo sem pool ativo.",
                e
            );
            // Em ambiente real de produção exigiriamos o banco, mas para fins de boot seguro mantemos o fallback ou saímos.
            std::process::exit(1);
        }
    };

    let repo = PostgresWalletRepository::new(pool);
    let state = AppState { repo };

    let app = Router::new()
        .route("/wallets", post(create_wallet))
        .route("/wallets/:id", get(get_wallet))
        .route("/wallets/:id/debit", post(debit_wallet))
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:8081")
        .await
        .unwrap();
    println!("Wallet Service API rodando em http://127.0.0.1:8081");

    axum::serve(listener, app).await.unwrap();
}
