// ~/nexavor/services/payment/src/domain/services_port.rs

use async_trait::async_trait;
use rust_decimal::Decimal;
use uuid::Uuid;

#[async_trait]
pub trait WalletPort: Send + Sync {
    async fn debit(&self, wallet_id: Uuid, amount: Decimal) -> Result<(), String>;
    async fn credit(&self, wallet_id: Uuid, amount: Decimal) -> Result<(), String>;
}

#[async_trait]
pub trait LedgerPort: Send + Sync {
    async fn record_transaction(
        &self,
        transaction_id: Uuid,
        source_account: Uuid,
        destination_account: Uuid,
        amount: Decimal,
        currency: String,
    ) -> Result<(), String>;
}
