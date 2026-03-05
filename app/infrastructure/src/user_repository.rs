use async_trait::async_trait;
use chrono::{DateTime, Utc};
use domain::user::{Credential, PasswordCredential, User, UserError, UserId, UserRepository};
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
struct UserWithCredentialRow {
    id: Uuid,
    name: String,
    email: String,
    created_at: DateTime<Utc>,
    password_hash: Option<String>,
    credential_created_at: Option<DateTime<Utc>>,
}

fn collect_user(rows: Vec<UserWithCredentialRow>) -> Option<User> {
    let first = rows.first()?;
    let credential = Credential::Password(PasswordCredential {
        password_hash: first.password_hash.clone()?,
        created_at: first.credential_created_at?,
    });
    Some(User {
        id: UserId::new(first.id),
        name: first.name.clone(),
        email: first.email.clone(),
        created_at: first.created_at,
        credential,
    })
}

const JOIN_SQL: &str = "
    SELECT u.id, u.name, u.email, u.created_at,
           c.password_hash, c.created_at AS credential_created_at
    FROM users u
    LEFT JOIN credentials c ON c.user_id = u.id";

#[async_trait]
impl UserRepository for PgUserRepository {
    async fn find_by_id(&self, id: &UserId) -> Result<Option<User>, UserError> {
        let rows =
            sqlx::query_as::<_, UserWithCredentialRow>(&format!("{JOIN_SQL} WHERE u.id = $1"))
                .bind(id.0)
                .fetch_all(&self.pool)
                .await
                .map_err(|e| UserError::Unexpected(e.to_string()))?;

        Ok(collect_user(rows))
    }

    async fn find_by_email(&self, email: &str) -> Result<Option<User>, UserError> {
        let rows =
            sqlx::query_as::<_, UserWithCredentialRow>(&format!("{JOIN_SQL} WHERE u.email = $1"))
                .bind(email)
                .fetch_all(&self.pool)
                .await
                .map_err(|e| UserError::Unexpected(e.to_string()))?;

        Ok(collect_user(rows))
    }

    async fn save(&self, user: &User) -> Result<(), UserError> {
        let mut tx = self
            .pool
            .begin()
            .await
            .map_err(|e| UserError::Unexpected(e.to_string()))?;

        sqlx::query("INSERT INTO users (id, name, email, created_at) VALUES ($1, $2, $3, $4)")
            .bind(user.id.0)
            .bind(&user.name)
            .bind(&user.email)
            .bind(user.created_at)
            .execute(&mut *tx)
            .await
            .map_err(|e| {
                if let sqlx::Error::Database(db_err) = &e
                    && db_err.constraint() == Some("users_email_key")
                {
                    return UserError::EmailAlreadyExists;
                }
                UserError::Unexpected(e.to_string())
            })?;

        let Credential::Password(pc) = &user.credential;
        sqlx::query(
            "INSERT INTO credentials (user_id, password_hash, created_at) VALUES ($1, $2, $3)",
        )
        .bind(user.id.0)
        .bind(&pc.password_hash)
        .bind(pc.created_at)
        .execute(&mut *tx)
        .await
        .map_err(|e| UserError::Unexpected(e.to_string()))?;

        tx.commit()
            .await
            .map_err(|e| UserError::Unexpected(e.to_string()))
    }
}
