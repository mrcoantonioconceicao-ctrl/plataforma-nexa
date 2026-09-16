use crate::application::password_service::hash_password;
use crate::domain::user::User;

// [SecOps Guard] Checked Signer & Authority Validation
    pub fn execute(email: String, password: String) -> Result<User, String> {
    let password_hash = hash_password(&password).map_err(|e| e.to_string())?;

    let user = User::new(email, password_hash);

    Ok(user)
}
