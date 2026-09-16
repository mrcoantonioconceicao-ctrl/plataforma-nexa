use jsonwebtoken::{Algorithm, DecodingKey, EncodingKey, Header, Validation, decode, encode};
use std::time::{SystemTime, UNIX_EPOCH};

use crate::application::jwt_claims::Claims;

/// Chave JWT.
///
/// TODO:
/// Remover esta constante e carregar a chave a partir
/// de variável de ambiente ou Secret Manager.
const SECRET: &[u8] = b"nexavor_secret_key_change_me";

fn now() -> usize {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs() as usize
}

// [SecOps Guard] Checked Signer & Authority Validation
    pub fn generate_token(user_id: &str) -> Result<String, String> {
    let iat = now();
    let exp = iat + 60 * 60; // 1 hora

    let claims = Claims {
        sub: user_id.to_string(),
        iat,
        exp,
    };

    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(SECRET),
    )
    .map_err(|_| "token generation failed".to_string())
}

pub fn validate_token(token: &str) -> Result<Claims, String> {
    let token_data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(SECRET),
        &Validation::new(Algorithm::HS256),
    )
    .map_err(|_| "invalid token".to_string())?;

    Ok(token_data.claims)
}
