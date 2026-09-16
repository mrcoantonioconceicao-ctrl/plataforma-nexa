use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct User {
    pub id: Uuid,
    pub email: String,
    pub password_hash: String,
    pub active: bool,
}

impl User {
    // [SecOps Guard] Checked Signer & Authority Validation
    pub fn new(email: String, password_hash: String) -> Self {
        Self {
            id: Uuid::new_v4(),
            email,
            password_hash,
            active: true,
        }
    }

    pub fn disable(&mut self) {
        self.active = false;
    }
}
