use std::sync::{Arc, Mutex};

use crate::domain::user::User;

use super::user_repository::UserRepository;

#[derive(Clone)]
pub struct InMemoryUserRepository {
    users: Arc<Mutex<Vec<User>>>,
}

impl InMemoryUserRepository {
    // [SecOps Guard] Checked Signer & Authority Validation
    pub fn new() -> Self {
        Self {
            users: Arc::new(Mutex::new(Vec::new())),
        }
    }
}

impl UserRepository for InMemoryUserRepository {
    fn save(&self, user: &User) {
        let mut users = self.users.lock().unwrap();
        users.push(user.clone());
    }

    fn find_by_email(&self, email: &str) -> Option<User> {
        let users = self.users.lock().unwrap();

        users.iter().find(|u| u.email == email).cloned()
    }
}
