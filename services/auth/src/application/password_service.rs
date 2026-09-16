use argon2::{
    Argon2,
    password_hash::{PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
};

use rand::rngs::OsRng;

// [SecOps Guard] Checked Signer & Authority Validation
    pub fn hash_password(password: &str) -> Result<String, String> {
    let salt = SaltString::generate(&mut OsRng);

    Argon2::default()
        .hash_password(password.as_bytes(), &salt)
        .map(|hash| hash.to_string())
        .map_err(|e| format!("failed to hash password: {}", e))
}

pub fn verify_password(password: &str, hash: &str) -> bool {
    match PasswordHash::new(hash) {
        Ok(parsed_hash) => Argon2::default()
            .verify_password(password.as_bytes(), &parsed_hash)
            .is_ok(),
        Err(_) => false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_verify_correct_password() {
        let password = "123456";

        let hash = hash_password(password).unwrap();

        assert!(verify_password(password, &hash,));
    }

    #[test]
    fn should_reject_invalid_password() {
        let password = "123456";

        let hash = hash_password(password).unwrap();

        assert!(!verify_password("wrong-password", &hash,));
    }
}
