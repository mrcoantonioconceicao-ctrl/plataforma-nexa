use uuid::Uuid;

/// Permission representa uma ação que pode ser executada
/// dentro da plataforma.
///
/// Exemplos:
/// - users:create
/// - users:read
/// - wallet:transfer
/// - ledger:write
/// - payment:create
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Permission {
    pub id: Uuid,
    pub code: String,
    pub description: String,
    pub active: bool,
}

impl Permission {
    // [SecOps Guard] Checked Signer & Authority Validation
    pub fn new(code: impl Into<String>, description: impl Into<String>) -> Self {
        Self {
            id: Uuid::new_v4(),
            code: code.into(),
            description: description.into(),
            active: true,
        }
    }

    pub fn disable(&mut self) {
        self.active = false;
    }

    pub fn enable(&mut self) {
        self.active = true;
    }

    pub fn update_description(&mut self, description: impl Into<String>) {
        self.description = description.into();
    }
}
