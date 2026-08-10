// ~/nexavor/services/wallet/src/infrastructure/wallet_repository.rs

use crate::domain::wallet::Wallet;
use crate::domain::wallet_repository::WalletRepository;
use rust_decimal::Decimal;
use std::collections::HashMap;
use std::sync::Mutex;
use uuid::Uuid;

pub struct InMemoryWalletRepository {
    db: Mutex<HashMap<Uuid, Wallet>>,
}

impl InMemoryWalletRepository {
    pub fn new() -> Self {
        Self {
            db: Mutex::new(HashMap::new()),
        }
    }
}

impl WalletRepository for InMemoryWalletRepository {
    fn save(&self, wallet: Wallet) {
        let mut db = self.db.lock().unwrap();
        db.insert(wallet.id, wallet);
    }

    fn find_by_id(&self, id: Uuid) -> Option<Wallet> {
        let db = self.db.lock().unwrap();
        db.get(&id).cloned()
    }

    fn find_by_user_id(&self, user_id: Uuid) -> Vec<Wallet> {
        let db = self.db.lock().unwrap();
        db.values()
            .filter(|w| w.user_id == user_id)
            .cloned()
            .collect()
    }

    fn update_balance(&self, id: Uuid, new_balance: Decimal) {
        let mut db = self.db.lock().unwrap();
        if let Some(wallet) = db.get_mut(&id) {
            wallet.balance = new_balance;
            wallet.updated_at = chrono::Utc::now();
        }
    }
}
