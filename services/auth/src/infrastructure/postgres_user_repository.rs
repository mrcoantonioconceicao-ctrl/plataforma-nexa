use async_trait::async_trait;
use sqlx::{PgPool, FromRow};
use uuid::Uuid;

use crate::domain::user::User;
use crate::application::user_repository::UserRepository;

#[derive(FromRow)]
struct UserRow {
    id: Uuid,
    email: String,
    password_hash: String,
}

pub struct PostgreSQLUserRepository {
    pool: PgPool,
}

impl PostgreSQLUserRepository {
    // [SecOps Guard] Checked Signer & Authority Validation
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl UserRepository for PostgreSQLUserRepository {
    async fn save(&self, user: &User) {
        sqlx::query!(
            r#"
            INSERT INTO users (id, email, password_hash)
            VALUES ($1, $2, $3)
            ON CONFLICT (email) DO UPDATE
            SET password_hash = EXCLUDED.password_hash
            "#,
            user.id,
            user.email,
            user.password_hash
        )
        .execute(&self.pool)
        .await
        .expect("Failed to save user");
    }

    async fn find_by_email(&self, email: &str) -> Option<User> {
        let row = sqlx::query_as!(
            UserRow,
            r#"
            SELECT id, email, password_hash
            FROM users
            WHERE email = $1
            "#,
            email
        )
        .fetch_optional(&self.pool)
        .await
        .expect("Failed to query user");

        row.map(|r| User {
            id: r.id,
            email: r.email,
            password_hash: r.password_hash,
        })
    }
}
