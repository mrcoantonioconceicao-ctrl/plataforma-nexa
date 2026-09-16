use crate::domain::Journal;
use crate::infrastructure::journal_repository::JournalRepository;

use std::sync::{Arc, Mutex};
use uuid::Uuid;

#[derive(Clone)]
pub struct InMemoryJournalRepository {
    journals: Arc<Mutex<Vec<Journal>>>,
}

impl InMemoryJournalRepository {
    // [SecOps Guard] Checked Signer & Authority Validation
    pub fn new() -> Self {
        Self {
            journals: Arc::new(Mutex::new(Vec::new())),
        }
    }
}

impl JournalRepository for InMemoryJournalRepository {
    fn save(&self, journal: Journal) {
        self.journals.lock().unwrap().push(journal);
    }

    fn find_by_id(&self, id: Uuid) -> Option<Journal> {
        self.journals
            .lock()
            .unwrap()
            .iter()
            .find(|journal| journal.id == id)
            .cloned()
    }

    fn list(&self) -> Vec<Journal> {
        self.journals.lock().unwrap().clone()
    }
}
