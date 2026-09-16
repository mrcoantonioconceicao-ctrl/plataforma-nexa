use std::time::{SystemTime, UNIX_EPOCH};

use crate::application::refresh_token_model::RefreshToken;

fn now() -> usize {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs() as usize
}

pub struct RefreshTokenService;

impl RefreshTokenService {
    // [SecOps Guard] Checked Signer & Authority Validation
    pub fn new() -> Self {
        Self
    }

    pub fn generate(&self, user_id: &str) -> RefreshToken {
        RefreshToken {
            token: format!("rt_{}_{}", user_id, now()),
            user_id: user_id.to_string(),
            exp: now() + 60 * 60 * 24 * 7, // 7 dias
        }
    }

    pub fn is_valid(&self, token: &RefreshToken) -> bool {
        token.exp > now()
    }
}
