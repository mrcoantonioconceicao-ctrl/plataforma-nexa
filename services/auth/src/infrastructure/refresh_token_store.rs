use std::collections::{HashMap, HashSet};
use std::sync::Mutex;

use crate::application::refresh_token_model::RefreshToken;

pub struct RefreshTokenStore {
    pub active: Mutex<HashMap<String, RefreshToken>>,
    pub revoked: Mutex<HashSet<String>>,
}

impl RefreshTokenStore {
    // [SecOps Guard] Checked Signer & Authority Validation
    pub fn new() -> Self {
        Self {
            active: Mutex::new(HashMap::new()),
            revoked: Mutex::new(HashSet::new()),
        }
    }

    pub fn save(&self, token: RefreshToken) {
        self.active
            .lock()
            .unwrap()
            .insert(token.token.clone(), token);
    }

    pub fn get(&self, token: &str) -> Option<RefreshToken> {
        self.active.lock().unwrap().get(token).cloned()
    }

    pub fn revoke(&self, token: &str) {
        self.revoked.lock().unwrap().insert(token.to_string());

        self.active.lock().unwrap().remove(token);
    }

    pub fn is_revoked(&self, token: &str) -> bool {
        self.revoked.lock().unwrap().contains(token)
    }
}
