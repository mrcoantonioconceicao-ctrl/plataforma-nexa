// ~/nexavor/services/wallet/src/domain/wallet.rs

use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Wallet {
    pub id: Uuid,
    pub user_id: Uuid,
    pub balance: Decimal,
    pub currency: String,
    pub created_at: DateTime<Utc>,
}

impl Wallet {
    pub fn new(user_id: Uuid, currency: String, initial_balance: Decimal) -> Self {
        Self {
            id: Uuid::new_v4(),
            user_id,
            balance: initial_balance,
            currency,
            created_at: Utc::now(),
        }
    }

    pub fn credit(&mut self, amount: Decimal) -> Result<(), String> {
        if amount <= Decimal::ZERO {
            return Err("O valor do crédito deve ser maior que zero.".to_string());
        }
        self.balance += amount;
        Ok(())
    }

    pub fn debit(&mut self, amount: Decimal) -> Result<(), String> {
        if amount <= Decimal::ZERO {
            return Err("O valor do débito deve ser maior que zero.".to_string());
        }
        if self.balance < amount {
            return Err("Saldo insuficiente na carteira.".to_string());
        }
        self.balance -= amount;
        Ok(())
    }
}
