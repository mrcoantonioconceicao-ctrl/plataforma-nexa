use axum::{
    extract::{Path, State},
    routing::{get, post},
    Json, Router,
};

use serde::{Deserialize, Serialize};
use std::net::SocketAddr;
use uuid::Uuid;

use ledger::{
    application::{
        chart_of_accounts_service::ChartOfAccountsService, journal_service::JournalService,
        ledger_service::LedgerService,
    },
    domain::{Account, Journal, JournalEntry, LedgerEntry},
    infrastructure::{
        database::Database, in_memory_journal_repository::InMemoryJournalRepository,
        postgres_ledger_repository::PostgresLedgerRepository,
    },
};

#[derive(Clone)]
struct AppState {
    ledger_service: LedgerService<PostgresLedgerRepository>,
    journal_service: JournalService,
    chart_service: ChartOfAccountsService,
}

#[derive(Serialize)]
struct HealthResponse {
    service: String,
    status: String,
}

#[derive(Serialize)]
struct AuditResponse {
    journals: usize,
    balanced: bool,
}

#[derive(Debug, Deserialize)]
struct CreateJournalRequest {
    entries: Vec<JournalEntry>,
}

async fn health() -> Json<HealthResponse> {
    Json(HealthResponse {
        service: "Nexavor Ledger".to_string(),
        status: "running".to_string(),
    })
}

async fn list_accounts(State(state): State<AppState>) -> Json<Vec<Account>> {
    Json(state.chart_service.all().into_iter().cloned().collect())
}

async fn create_journal(
    State(state): State<AppState>,
    Json(payload): Json<CreateJournalRequest>,
) -> Result<Json<Journal>, String> {
    let journal = Journal::new(payload.entries);

    let created = state.journal_service.create(journal)?;

    state.ledger_service.post_journal(&created)?;

    Ok(Json(created))
}

async fn list_journals(State(state): State<AppState>) -> Json<Vec<Journal>> {
    Json(state.journal_service.list())
}

async fn get_journal(
    Path(id): Path<String>,
    State(state): State<AppState>,
) -> Result<Json<Journal>, String> {
    let uuid = Uuid::parse_str(&id).map_err(|_| "UUID inválido".to_string())?;

    match state.journal_service.find(uuid) {
        Some(journal) => Ok(Json(journal)),
        None => Err("Journal não encontrado.".into()),
    }
}

async fn list_ledger(State(state): State<AppState>) -> Json<Vec<LedgerEntry>> {
    Json(state.ledger_service.list_all())
}

async fn get_ledger_entry(
    Path(id): Path<String>,
    State(state): State<AppState>,
) -> Result<Json<LedgerEntry>, String> {
    let uuid = Uuid::parse_str(&id).map_err(|_| "UUID inválido".to_string())?;

    match state.ledger_service.find_by_id(uuid) {
        Some(entry) => Ok(Json(entry)),
        None => Err("Lançamento não encontrado.".into()),
    }
}

async fn list_account_entries(
    Path(account): Path<String>,
    State(state): State<AppState>,
) -> Json<Vec<LedgerEntry>> {
    Json(state.ledger_service.find_by_account(&account))
}

async fn audit_ledger(State(state): State<AppState>) -> Json<AuditResponse> {
    let journals = state.journal_service.list();

    let balanced = journals.iter().all(|journal| journal.validate().is_ok());

    Json(AuditResponse {
        journals: journals.len(),
        balanced,
    })
}

#[tokio::main]
async fn main() {
    let database = Database::connect_from_env()
        .await
        .expect("Falha ao conectar ao PostgreSQL para o Ledger");

    let ledger_repository = PostgresLedgerRepository::new(database.pool);
    let journal_repository = InMemoryJournalRepository::new();

    let state = AppState {
        ledger_service: LedgerService::new(ledger_repository),
        journal_service: JournalService::new(journal_repository),
        chart_service: ChartOfAccountsService::new(),
    };

    let app = Router::new()
        .route("/", get(health))
        .route("/accounts", get(list_accounts))
        .route("/journals", get(list_journals))
        .route("/journals", post(create_journal))
        .route("/journals/:id", get(get_journal))
        .route("/ledger", get(list_ledger))
        .route("/ledger/:id", get(get_ledger_entry))
        .route("/ledger/account/:account", get(list_account_entries))
        .route("/ledger/audit", get(audit_ledger))
        .with_state(state);

    let addr = SocketAddr::from(([0, 0, 0, 0], 4002));

    println!("Nexavor Ledger running with PostgreSQL on {}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();

    axum::serve(listener, app).await.unwrap();
}
