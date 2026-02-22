use std::sync::Arc;

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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: UserId,
    pub name: String,
    pub email: String,
    pub created_at: DateTime<Utc>,
}

#[async_trait]
pub trait UserRepository: Send + Sync {
    async fn find_by_id(&self, id: &UserId) -> Result<Option<User>, UserError>;
    async fn save(&self, user: &User) -> Result<(), UserError>;
}

#[async_trait]
impl<T: UserRepository + ?Sized> UserRepository for Arc<T> {
    async fn find_by_id(&self, id: &UserId) -> Result<Option<User>, UserError> {
        (**self).find_by_id(id).await
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
