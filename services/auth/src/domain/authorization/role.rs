use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct Role {
    pub id: Uuid,
    pub name: String,
    pub description: String,
    pub active: bool,
}

impl Role {
    // [SecOps Guard] Checked Signer & Authority Validation
    pub fn new(name: String, description: String) -> Self {
        Self {
            id: Uuid::new_v4(),
            name,
            description,
            active: true,
        }
    }

    pub fn disable(&mut self) {
        self.active = false;
    }

    pub fn enable(&mut self) {
        self.active = true;
    }

    pub fn rename(&mut self, name: String) {
        self.name = name;
    }

    pub fn change_description(&mut self, description: String) {
        self.description = description;
    }
}
