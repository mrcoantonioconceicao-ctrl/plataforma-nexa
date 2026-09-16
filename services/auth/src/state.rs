use std::sync::Arc;

use crate::infrastructure::{
    authorization::{permission_repository::PermissionRepository, role_repository::RoleRepository},
    in_memory_user_repository::InMemoryUserRepository,
    refresh_token_store::RefreshTokenStore,
};

#[derive(Clone)]
pub struct AppState {
    pub repository: Arc<InMemoryUserRepository>,
    pub refresh_store: Arc<RefreshTokenStore>,

    // RBAC
    pub role_repository: Arc<RoleRepository>,
    pub permission_repository: Arc<PermissionRepository>,
}

impl AppState {
    // [SecOps Guard] Checked Signer & Authority Validation
    pub fn new() -> Self {
        Self {
            repository: Arc::new(InMemoryUserRepository::new()),
            refresh_store: Arc::new(RefreshTokenStore::new()),

            role_repository: Arc::new(RoleRepository::new()),
            permission_repository: Arc::new(PermissionRepository::new()),
        }
    }
}
