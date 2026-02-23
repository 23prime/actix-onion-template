use async_trait::async_trait;
use chrono::{DateTime, Utc};
use domain::user::{User, UserError, UserId, UserRepository};
use sqlx::{FromRow, PgPool};
use uuid::Uuid;

pub struct PgUserRepository {
    pool: PgPool,
}

impl PgUserRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[derive(FromRow)]
struct UserRow {
    id: Uuid,
    name: String,
    email: String,
    created_at: DateTime<Utc>,
}

impl From<UserRow> for User {
    fn from(row: UserRow) -> Self {
        User {
            id: UserId::new(row.id),
            name: row.name,
            email: row.email,
            created_at: row.created_at,
        }
    }
}

#[async_trait]
impl UserRepository for PgUserRepository {
    async fn find_by_id(&self, id: &UserId) -> Result<Option<User>, UserError> {
        let row = sqlx::query_as::<_, UserRow>(
            "SELECT id, name, email, created_at FROM users WHERE id = $1",
        )
        .bind(id.0)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| UserError::Unexpected(e.to_string()))?;

        Ok(row.map(User::from))
    }

    async fn find_by_email(&self, email: &str) -> Result<Option<User>, UserError> {
        let row = sqlx::query_as::<_, UserRow>(
            "SELECT id, name, email, created_at FROM users WHERE email = $1",
        )
        .bind(email)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| UserError::Unexpected(e.to_string()))?;

        Ok(row.map(User::from))
    }

    async fn save(&self, user: &User) -> Result<(), UserError> {
        sqlx::query("INSERT INTO users (id, name, email, created_at) VALUES ($1, $2, $3, $4)")
            .bind(user.id.0)
            .bind(&user.name)
            .bind(&user.email)
            .bind(user.created_at)
            .execute(&self.pool)
            .await
            .map_err(|e| {
                if let sqlx::Error::Database(db_err) = &e
                    && db_err.constraint() == Some("users_email_key")
                {
                    return UserError::EmailAlreadyExists;
                }
                UserError::Unexpected(e.to_string())
            })?;

        Ok(())
    }
}
