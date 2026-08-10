use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LedgerEntry {
    pub id: Uuid,

    pub journal_id: Uuid,

    pub account_id: Uuid,

    pub currency: String,

    pub debit: Decimal,

    pub credit: Decimal,

    pub created_at: DateTime<Utc>,
}

impl LedgerEntry {
    pub fn new(
        journal_id: Uuid,
        account_id: Uuid,
        currency: String,
        debit: Decimal,
        credit: Decimal,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),

            journal_id,

            account_id,

            currency,

            debit,

            credit,

            created_at: Utc::now(),
        }
    }

    pub fn is_debit(&self) -> bool {
        self.debit > Decimal::ZERO
    }

    pub fn is_credit(&self) -> bool {
        self.credit > Decimal::ZERO
    }

    pub fn amount(&self) -> Decimal {
        if self.is_debit() {
            self.debit
        } else {
            self.credit
        }
    }
}
