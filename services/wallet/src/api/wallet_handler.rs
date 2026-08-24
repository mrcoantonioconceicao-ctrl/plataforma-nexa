// ~/nexavor/services/wallet/src/api/wallet_handler.rs

use crate::domain::wallet::Wallet;
use crate::infrastructure::postgres_wallet_repository::PostgresWalletRepository;
use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

// Estado compartilhado contendo o repositório real do PostgreSQL
#[derive(Clone)]
pub struct AppState {
    pub repo: PostgresWalletRepository,
}

#[derive(Deserialize)]
pub struct CreateWalletRequest {
    pub user_id: Uuid,
    pub currency: String,
    pub initial_balance: Decimal,
}

#[derive(Deserialize)]
pub struct TransactionRequest {
    pub amount: Decimal,
}

#[derive(Serialize)]
pub struct WalletResponse {
    pub id: Uuid,
    pub user_id: Uuid,
    pub balance: Decimal,
    pub currency: String,
}

impl From<Wallet> for WalletResponse {
    fn from(w: Wallet) -> Self {
        Self {
            id: w.id,
            user_id: w.user_id,
            balance: w.balance,
            currency: w.currency,
        }
    }
}

// Handler: Criar Carteira (Persistida no PostgreSQL)
pub async fn create_wallet(
    State(state): State<AppState>,
    Json(payload): Json<CreateWalletRequest>,
) -> Result<(StatusCode, Json<WalletResponse>), StatusCode> {
    let wallet = Wallet::new(payload.user_id, payload.currency, payload.initial_balance);

    match state.repo.save(&wallet).await {
        Ok(_) => Ok((StatusCode::CREATED, Json(wallet.into()))),
        Err(e) => {
            eprintln!("Erro ao salvar carteira no banco: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

// Handler: Consultar Carteira por ID no PostgreSQL
pub async fn get_wallet(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Json<WalletResponse>, StatusCode> {
    match state.repo.find_by_id(id).await {
        Ok(Some(wallet)) => Ok(Json(wallet.into())),
        Ok(None) => Err(StatusCode::NOT_FOUND),
        Err(e) => {
            eprintln!("Erro ao buscar carteira no banco: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

// Handler: Debitar da Carteira (Atualizando estado e persistência)
pub async fn debit_wallet(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    Json(payload): Json<TransactionRequest>,
) -> Result<Json<WalletResponse>, StatusCode> {
    let mut wallet = match state.repo.find_by_id(id).await {
        Ok(Some(w)) => w,
        Ok(None) => return Err(StatusCode::NOT_FOUND),
        Err(_) => return Err(StatusCode::INTERNAL_SERVER_ERROR),
    };

    if let Err(_) = wallet.debit(payload.amount) {
        return Err(StatusCode::BAD_REQUEST);
    }

    match state.repo.save(&wallet).await {
        Ok(_) => Ok(Json(wallet.into())),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}
