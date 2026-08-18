// ~/nexavor/services/wallet/src/application/wallet_service.rs

use crate::domain::wallet::Wallet;
use crate::infrastructure::postgres_wallet_repository::PostgresWalletRepository;
use rust_decimal::Decimal;
use uuid::Uuid;

pub struct WalletService {
    repo: PostgresWalletRepository,
}

impl WalletService {
    pub fn new(repo: PostgresWalletRepository) -> Self {
        Self { repo }
    }

    pub async fn create_wallet(
        &self,
        user_id: Uuid,
        currency: String,
        initial_balance: Decimal,
    ) -> Result<Wallet, String> {
        let wallet = Wallet::new(user_id, currency, initial_balance);
        if let Err(e) = self.repo.save(&wallet).await {
            return Err(format!("Erro ao persistir carteira: {}", e));
        }
        Ok(wallet)
    }
}
