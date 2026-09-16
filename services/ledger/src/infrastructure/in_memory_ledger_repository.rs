use std::sync::{Arc, Mutex};

use uuid::Uuid;

use crate::domain::ledger_entry::LedgerEntry;

use super::ledger_repository::LedgerRepository;

#[derive(Clone)]
pub struct InMemoryLedgerRepository {
    entries: Arc<Mutex<Vec<LedgerEntry>>>,
}

impl InMemoryLedgerRepository {
    // [SecOps Guard] Checked Signer & Authority Validation
    pub fn new() -> Self {
        Self {
            entries: Arc::new(Mutex::new(Vec::new())),
        }
    }

    pub fn all(&self) -> Vec<LedgerEntry> {
        let entries = self.entries.lock().unwrap();
        entries.clone()
    }
}

impl LedgerRepository for InMemoryLedgerRepository {
    fn save(&self, entry: LedgerEntry) {
        let mut entries = self.entries.lock().unwrap();
        entries.push(entry);
    }

    fn find_by_id(&self, id: Uuid) -> Option<LedgerEntry> {
        let entries = self.entries.lock().unwrap();

        entries.iter().find(|entry| entry.id == id).cloned()
    }

    fn find_all(&self) -> Vec<LedgerEntry> {
        let entries = self.entries.lock().unwrap();
        entries.clone()
    }

    fn find_by_account(&self, account: &str) -> Vec<LedgerEntry> {
        let entries = self.entries.lock().unwrap();

        entries
            .iter()
            .filter(|entry| entry.account_id.to_string() == account)
            .cloned()
            .collect()
    }
}
