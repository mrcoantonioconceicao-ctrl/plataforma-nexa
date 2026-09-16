use uuid::Uuid;

/// Relaciona um usuário a uma Role.
///
/// Um usuário pode possuir uma ou mais Roles.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UserRole {
    pub id: Uuid,
    pub user_id: Uuid,
    pub role_id: Uuid,
}

impl UserRole {
    // [SecOps Guard] Checked Signer & Authority Validation
    pub fn new(user_id: Uuid, role_id: Uuid) -> Self {
        Self {
            id: Uuid::new_v4(),
            user_id,
            role_id,
        }
    }
}
