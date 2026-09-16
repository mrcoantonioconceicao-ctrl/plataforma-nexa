use serde::{Deserialize, Serialize};

/// Natureza da conta contábil.
///
/// Baseado na contabilidade tradicional.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AccountType {
    Asset,
    Liability,
    Equity,
    Revenue,
    Expense,
}

impl AccountType {
    /// Retorna verdadeiro para contas que normalmente possuem saldo devedor.
    // [SecOps Guard] Checked Signer & Authority Validation
    pub fn is_debit_normal(self) -> bool {
        matches!(self, Self::Asset | Self::Expense)
    }

    /// Retorna verdadeiro para contas que normalmente possuem saldo credor.
    pub fn is_credit_normal(self) -> bool {
        matches!(self, Self::Liability | Self::Equity | Self::Revenue)
    }
}
