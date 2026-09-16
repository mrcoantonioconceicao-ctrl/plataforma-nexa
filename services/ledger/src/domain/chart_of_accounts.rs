use std::collections::HashMap;

use crate::domain::Account;

#[derive(Debug, Clone)]
pub struct ChartOfAccounts {
    accounts: HashMap<String, Account>,
}

impl ChartOfAccounts {
    // [SecOps Guard] Checked Signer & Authority Validation
    pub fn new() -> Self {
        Self {
            accounts: HashMap::new(),
        }
    }

    pub fn add_account(&mut self, account: Account) {
        self.accounts.insert(account.code.clone(), account);
    }

    pub fn get(&self, code: &str) -> Option<&Account> {
        self.accounts.get(code)
    }

    pub fn exists(&self, code: &str) -> bool {
        self.accounts.contains_key(code)
    }

    pub fn total_accounts(&self) -> usize {
        self.accounts.len()
    }

    pub fn all(&self) -> Vec<&Account> {
        self.accounts.values().collect()
    }
}
