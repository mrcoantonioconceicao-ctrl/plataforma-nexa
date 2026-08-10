// ~/nexavor/services/wallet/src/main.rs

use axum::{
    extract::{Path, State},
    routing::{get, post},
    Json, Router,
};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use std::{net::SocketAddr, sync::Arc};
use uuid::Uuid;

mod application;
mod domain;
mod infrastructure;

use application::wallet_service::WalletService;
use infrastructure::wallet_repository::InMemoryWalletRepository;

#[derive(Clone)]
struct AppState {
    service: Arc<WalletService<InMemoryWalletRepository>>,
}

impl AppState {
    fn new() -> Self {
        let repo = InMemoryWalletRepository::new();
        let service = WalletService::new(repo);

        Self {
            service: Arc::new(service),
        }
    }
}

#[derive(Deserialize)]
struct CreateWalletRequest {
    user_id: Uuid,
    currency: String,
}

#[derive(Deserialize)]
struct AmountRequest {
    wallet_id: Uuid,
    amount: Decimal,
}

#[derive(Serialize)]
struct WalletResponse {
    id: Uuid,
    user_id: Uuid,
    currency: String,
    balance: Decimal,
}

async fn create_wallet(
    State(state): State<AppState>,
    Json(req): Json<CreateWalletRequest>,
) -> Json<WalletResponse> {
    let wallet = state.service.create_wallet(req.user_id, req.currency);

    Json(WalletResponse {
        id: wallet.id,
        user_id: wallet.user_id,
        currency: wallet.currency,
        balance: wallet.balance,
    })
}

async fn credit(
    State(state): State<AppState>,
    Json(req): Json<AmountRequest>,
) -> Result<Json<WalletResponse>, String> {
    let wallet = state
        .service
        .credit(req.wallet_id, req.amount)
        .map_err(|e| e)?;

    Ok(Json(WalletResponse {
        id: wallet.id,
        user_id: wallet.user_id,
        currency: wallet.currency,
        balance: wallet.balance,
    }))
}

async fn debit(
    State(state): State<AppState>,
    Json(req): Json<AmountRequest>,
) -> Result<Json<WalletResponse>, String> {
    let wallet = state
        .service
        .debit(req.wallet_id, req.amount)
        .map_err(|e| e)?;

    Ok(Json(WalletResponse {
        id: wallet.id,
        user_id: wallet.user_id,
        currency: wallet.currency,
        balance: wallet.balance,
    }))
}

async fn get_wallet(
    State(state): State<AppState>,
    Path(user_id_str): Path<String>,
) -> Result<Json<Vec<WalletResponse>>, String> {
    let user_id =
        Uuid::parse_str(&user_id_str).map_err(|_| "UUID de usuário inválido".to_string())?;
    let wallets = state.service.get_wallets_by_user(user_id);

    let response = wallets
        .into_iter()
        .map(|w| WalletResponse {
            id: w.id,
            user_id: w.user_id,
            currency: w.currency,
            balance: w.balance,
        })
        .collect();

    Ok(Json(response))
}

#[tokio::main]
async fn main() {
    let state = AppState::new();

    let app = Router::new()
        .route("/wallets", post(create_wallet))
        .route("/wallets/credit", post(credit))
        .route("/wallets/debit", post(debit))
        .route("/wallets/user/:user_id", get(get_wallet))
        .with_state(state);

    let addr = SocketAddr::from(([0, 0, 0, 0], 4003));

    println!("Nexavor Wallet Service running on {}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
