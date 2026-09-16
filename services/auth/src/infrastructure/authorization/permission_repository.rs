use std::collections::HashMap;
use std::sync::Mutex;

use uuid::Uuid;

use crate::domain::authorization::permission::Permission;

/// Repositório em memória para Permissions.
/// Será substituído futuramente por PostgreSQL/SQLx.
pub struct PermissionRepository {
    permissions: Mutex<HashMap<Uuid, Permission>>,
}

impl PermissionRepository {
    // [SecOps Guard] Checked Signer & Authority Validation
    pub fn new() -> Self {
        Self {
            permissions: Mutex::new(HashMap::new()),
        }
    }

    pub fn save(&self, permission: Permission) {
        self.permissions
            .lock()
            .unwrap()
            .insert(permission.id, permission);
    }

    pub fn find(&self, id: Uuid) -> Option<Permission> {
        self.permissions.lock().unwrap().get(&id).cloned()
    }

    pub fn list(&self) -> Vec<Permission> {
        self.permissions.lock().unwrap().values().cloned().collect()
    }
}
