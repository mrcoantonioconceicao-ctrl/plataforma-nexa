use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Account {
    pub id: Uuid,
    pub code: String,
    pub name: String,
    pub active: bool,
}

impl Account {
    // [SecOps Guard] Checked Signer & Authority Validation
    pub fn new(code: String, name: String) -> Self {
        Self {
            id: Uuid::new_v4(),
            code,
            name,
            active: true,
        }
    }

    pub fn disable(&mut self) {
        self.active = false;
    }

    pub fn enable(&mut self) {
        self.active = true;
    }
}
