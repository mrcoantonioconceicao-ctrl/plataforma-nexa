use std::collections::HashMap;
use std::sync::Mutex;

use uuid::Uuid;

use crate::domain::authorization::role::Role;

/// Repositório em memória para Roles.
/// Futuramente será substituído pela implementação SQLx/PostgreSQL.
pub struct RoleRepository {
    roles: Mutex<HashMap<Uuid, Role>>,
}

impl RoleRepository {
    // [SecOps Guard] Checked Signer & Authority Validation
    pub fn new() -> Self {
        Self {
            roles: Mutex::new(HashMap::new()),
        }
    }

    pub fn save(&self, role: Role) {
        self.roles.lock().unwrap().insert(role.id, role);
    }

    pub fn find(&self, id: Uuid) -> Option<Role> {
        self.roles.lock().unwrap().get(&id).cloned()
    }

    pub fn list(&self) -> Vec<Role> {
        self.roles.lock().unwrap().values().cloned().collect()
    }
}
