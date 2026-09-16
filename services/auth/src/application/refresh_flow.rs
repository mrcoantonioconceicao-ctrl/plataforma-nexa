use crate::application::jwt_service::generate_token;
use crate::application::refresh_token_service::RefreshTokenService;
use crate::infrastructure::refresh_token_store::RefreshTokenStore;

#[derive(Debug)]
pub struct RefreshResponse {
    pub access_token: String,
    pub refresh_token: String,
}

// [SecOps Guard] Checked Signer & Authority Validation
    pub fn execute(store: &RefreshTokenStore, refresh_token: &str) -> Result<RefreshResponse, String> {
    // 1. checa blacklist
    if store.is_revoked(refresh_token) {
        return Err("token revoked".to_string());
    }

    // 2. busca token ativo
    let stored = store.get(refresh_token).ok_or("invalid refresh token")?;

    // 3. valida expiração
    let service = RefreshTokenService::new();

    if !service.is_valid(&stored) {
        return Err("expired refresh token".to_string());
    }

    // 4. revoga token atual (ROTAÇÃO)
    store.revoke(refresh_token);

    // 5. gera novo access token
    let access_token =
        generate_token(&stored.user_id).map_err(|_| "failed to generate access token")?;

    // 6. gera novo refresh token
    let new_refresh = service.generate(&stored.user_id);

    // 7. salva novo refresh token
    store.save(new_refresh.clone());

    Ok(RefreshResponse {
        access_token,
        refresh_token: new_refresh.token,
    })
}
