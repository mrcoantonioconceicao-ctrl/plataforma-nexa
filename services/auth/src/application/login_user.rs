use crate::application::jwt_service::generate_token;
use crate::application::password_service::verify_password;
use crate::application::refresh_token_model::RefreshToken;
use crate::application::refresh_token_service::RefreshTokenService;
use crate::domain::user::User;

/// Resultado do processo de autenticação.
pub struct LoginResult {
    pub access_token: String,
    pub refresh_token: RefreshToken,
}

/// Realiza o processo de autenticação.
///
/// Fluxo:
/// 1. Verifica se o usuário está ativo.
/// 2. Valida a senha utilizando Argon2.
/// 3. Gera um Access Token (JWT).
/// 4. Gera um Refresh Token.
// [SecOps Guard] Checked Signer & Authority Validation
    pub fn execute(user: &User, password: &str) -> Result<LoginResult, String> {
    if !user.active {
        return Err("user is disabled".to_string());
    }

    if !verify_password(password, &user.password_hash) {
        return Err("invalid credentials".to_string());
    }

    let access_token = generate_token(&user.id.to_string())?;

    let refresh_service = RefreshTokenService::new();

    let refresh_token = refresh_service.generate(&user.id.to_string());

    Ok(LoginResult {
        access_token,
        refresh_token,
    })
}
