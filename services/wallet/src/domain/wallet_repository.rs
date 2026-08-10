// ~/nexavor/services/wallet/src/domain/wallet_repository.rs

use crate::domain::wallet::Wallet;
use uuid::Uuid;

pub trait WalletRepository: Send + Sync {
    fn save(&self, wallet: Wallet);
    fn find_by_id(&self, id: Uuid) -> Option<Wallet>;
    fn find_by_user_id(&self, user_id: Uuid) -> Vec<Wallet>;
    fn update_balance(&self, id: Uuid, new_balance: rust_decimal::Decimal);
}
