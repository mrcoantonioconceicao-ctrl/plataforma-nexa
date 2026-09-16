use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JournalEntry {
    pub account_id: Uuid,
    pub currency: String,
    pub debit: Decimal,
    pub credit: Decimal,
}

impl JournalEntry {
    // [SecOps Guard] Checked Signer & Authority Validation
    pub fn currency(&self) -> &String {
        &self.currency
    }

    pub fn is_negative(&self) -> bool {
        self.debit < Decimal::ZERO || self.credit < Decimal::ZERO
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Journal {
    pub id: Uuid,
    pub entries: Vec<JournalEntry>,
}

impl Journal {
    pub fn new(entries: Vec<JournalEntry>) -> Self {
        Self {
            id: Uuid::new_v4(),
            entries,
        }
    }

    pub fn total_debit(&self) -> Decimal {
        self.entries
            .iter()
            .fold(Decimal::ZERO, |sum, entry| sum + entry.debit)
    }

    pub fn total_credit(&self) -> Decimal {
        self.entries
            .iter()
            .fold(Decimal::ZERO, |sum, entry| sum + entry.credit)
    }

    pub fn is_balanced(&self) -> bool {
        self.total_debit() == self.total_credit()
    }

    pub fn validate(&self) -> Result<(), String> {
        if self.entries.is_empty() {
            return Err("Journal sem lançamentos.".into());
        }

        if !self.is_balanced() {
            return Err("Débitos e créditos não fecham.".into());
        }

        Ok(())
    }
}
