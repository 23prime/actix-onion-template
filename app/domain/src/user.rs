use std::sync::Arc;

use argon2::{
    Argon2,
    password_hash::{PasswordHash, PasswordHasher, PasswordVerifier, SaltString, rand_core::OsRng},
};
use async_trait::async_trait;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct UserId(pub Uuid);

impl UserId {
    pub fn new(id: Uuid) -> Self {
        Self(id)
    }
}

#[derive(Debug, Clone)]
pub struct PasswordCredential {
    pub password_hash: String,
    pub created_at: DateTime<Utc>,
}

impl PasswordCredential {
    pub fn new(password: &str) -> Result<Self, UserError> {
        let salt = SaltString::generate(&mut OsRng);
        let password_hash = Argon2::default()
            .hash_password(password.as_bytes(), &salt)
            .map_err(|e| UserError::Unexpected(e.to_string()))?
            .to_string();
        Ok(Self {
            password_hash,
            created_at: Utc::now(),
        })
    }

    pub fn verify(&self, password: &str) -> bool {
        let Ok(parsed_hash) = PasswordHash::new(&self.password_hash) else {
            return false;
        };
        Argon2::default()
            .verify_password(password.as_bytes(), &parsed_hash)
            .is_ok()
    }
}

#[derive(Debug, Clone)]
pub enum Credential {
    Password(PasswordCredential),
}

#[derive(Debug, Clone)]
pub struct User {
    pub id: UserId,
    pub name: String,
    pub email: String,
    pub created_at: DateTime<Utc>,
    pub credentials: Vec<Credential>,
}

impl User {
    pub fn verify_password(&self, password: &str) -> bool {
        self.credentials.iter().any(|c| match c {
            Credential::Password(pc) => pc.verify(password),
        })
    }
}

#[async_trait]
pub trait UserRepository: Send + Sync {
    async fn find_by_id(&self, id: &UserId) -> Result<Option<User>, UserError>;
    async fn find_by_email(&self, email: &str) -> Result<Option<User>, UserError>;
    async fn save(&self, user: &User) -> Result<(), UserError>;
}

#[async_trait]
impl<T: UserRepository + ?Sized> UserRepository for Arc<T> {
    async fn find_by_id(&self, id: &UserId) -> Result<Option<User>, UserError> {
        (**self).find_by_id(id).await
    }

    async fn find_by_email(&self, email: &str) -> Result<Option<User>, UserError> {
        (**self).find_by_email(email).await
    }

    async fn save(&self, user: &User) -> Result<(), UserError> {
        (**self).save(user).await
    }
}

#[derive(Debug)]
pub enum UserError {
    NotFound,
    EmailAlreadyExists,
    Unexpected(String),
}

impl std::fmt::Display for UserError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            UserError::NotFound => write!(f, "user not found"),
            UserError::EmailAlreadyExists => write!(f, "email already exists"),
            UserError::Unexpected(msg) => write!(f, "unexpected error: {msg}"),
        }
    }
}

impl std::error::Error for UserError {}
