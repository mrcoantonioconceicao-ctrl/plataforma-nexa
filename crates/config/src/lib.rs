// ============================================================================
// [AST REFACTORED] MÓDULO REMEDIADO DETERMINISTICAMENTE (CLEAN CODE & DDD)
// Arquivo: crates/config/src/lib.rs | Conformidade com Restrições da AST
// ============================================================================

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Environment {
    Development,
    Staging,
    Production,
}

impl Environment {
    pub fn from_str(value: &str) -> Self {
        match value.to_lowercase().as_str() {
            "production" => Self::Production,

            "staging" => Self::Staging,

            _ => Self::Development,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    pub app_name: String,

    pub app_env: Environment,

    pub app_port: u16,

    pub jwt_secret: String,

    pub jwt_expiration_seconds: usize,

    pub refresh_token_expiration_seconds: usize,
}

impl AppConfig {
    pub fn load() -> Self {
        dotenvy::dotenv().ok();

        let app_name = std::env::var("APP_NAME").unwrap_or_else(|_| "BlockX".to_string());

        let app_env = std::env::var("APP_ENV").unwrap_or_else(|_| "development".to_string());

        let app_port = std::env::var("APP_PORT")
            .unwrap_or_else(|_| "8080".to_string())
            .parse::<u16>()
            .unwrap_or(8080);

        let jwt_secret =
            std::env::var("JWT_SECRET").unwrap_or_else(|_| "blockx-development-secret".to_string());

        let jwt_expiration_seconds = std::env::var("JWT_EXPIRATION_SECONDS")
            .unwrap_or_else(|_| "900".to_string())
            .parse::<usize>()
            .unwrap_or(900);

        let refresh_token_expiration_seconds = std::env::var("REFRESH_TOKEN_EXPIRATION_SECONDS")
            .unwrap_or_else(|_| "604800".to_string())
            .parse::<usize>()
            .unwrap_or(604800);

        Self {
            app_name,

            app_env: Environment::from_str(&app_env),

            app_port,

            jwt_secret,

            jwt_expiration_seconds,

            refresh_token_expiration_seconds,
        }
    }
}
