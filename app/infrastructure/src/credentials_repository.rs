use async_trait::async_trait;
use chrono::{DateTime, Utc};
use domain::{
    auth_error::AuthError, credentials::Credentials, credentials_repository::CredentialsRepository,
    user::UserId,
};
use sqlx::{FromRow, PgPool};
use uuid::Uuid;

pub struct PgCredentialsRepository {
    pool: PgPool,
}

impl PgCredentialsRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[derive(FromRow)]
struct CredentialsRow {
    user_id: Uuid,
    password_hash: String,
    created_at: DateTime<Utc>,
}

impl From<CredentialsRow> for Credentials {
    fn from(row: CredentialsRow) -> Self {
        Credentials {
            user_id: UserId::new(row.user_id),
            password_hash: row.password_hash,
            created_at: row.created_at,
        }
    }
}

#[async_trait]
impl CredentialsRepository for PgCredentialsRepository {
    async fn find_by_user_id(&self, user_id: &UserId) -> Result<Option<Credentials>, AuthError> {
        let row = sqlx::query_as::<_, CredentialsRow>(
            "SELECT user_id, password_hash, created_at FROM credentials WHERE user_id = $1",
        )
        .bind(user_id.0)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| AuthError::Unexpected(e.to_string()))?;

        Ok(row.map(Credentials::from))
    }

    async fn save(&self, credentials: &Credentials) -> Result<(), AuthError> {
        sqlx::query(
            "INSERT INTO credentials (user_id, password_hash, created_at) VALUES ($1, $2, $3)",
        )
        .bind(credentials.user_id.0)
        .bind(&credentials.password_hash)
        .bind(credentials.created_at)
        .execute(&self.pool)
        .await
        .map_err(|e| AuthError::Unexpected(e.to_string()))?;

        Ok(())
    }
}
