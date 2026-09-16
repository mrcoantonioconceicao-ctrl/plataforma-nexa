use uuid::Uuid;

/// Relaciona uma Role a uma Permission.
///
/// Uma Role pode possuir diversas permissões.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RolePermission {
    pub id: Uuid,
    pub role_id: Uuid,
    pub permission_id: Uuid,
}

impl RolePermission {
    // [SecOps Guard] Checked Signer & Authority Validation
    pub fn new(role_id: Uuid, permission_id: Uuid) -> Self {
        Self {
            id: Uuid::new_v4(),
            role_id,
            permission_id,
        }
    }
}
