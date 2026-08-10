// ~/nexavor/services/wallet/src/application/wallet_service.rs

use crate::domain::wallet::Wallet;
use crate::domain::wallet_repository::WalletRepository;
use rust_decimal::Decimal;
use uuid::Uuid;

pub struct WalletService<R: WalletRepository> {
    repo: R,
}

impl<R: WalletRepository> WalletService<R> {
    pub fn new(repo: R) -> Self {
        Self { repo }
    }

    pub fn create_wallet(&self, user_id: Uuid, currency: String) -> Wallet {
        let wallet = Wallet::new(user_id, currency);
        self.repo.save(wallet.clone());
        wallet
    }

    pub fn get_wallets_by_user(&self, user_id: Uuid) -> Vec<Wallet> {
        self.repo.find_by_user_id(user_id)
    }

    pub fn credit(&self, wallet_id: Uuid, amount: Decimal) -> Result<Wallet, String> {
        let mut wallet = self
            .repo
            .find_by_id(wallet_id)
            .ok_or("Wallet não encontrada.")?;

        wallet.credit(amount)?;
        self.repo.update_balance(wallet.id, wallet.balance);

        Ok(wallet)
    }

    pub fn debit(&self, wallet_id: Uuid, amount: Decimal) -> Result<Wallet, String> {
        let mut wallet = self
            .repo
            .find_by_id(wallet_id)
            .ok_or("Wallet não encontrada.")?;

        wallet.debit(amount)?;
        self.repo.update_balance(wallet.id, wallet.balance);

        Ok(wallet)
    }
}
