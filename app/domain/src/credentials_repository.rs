use std::sync::Arc;

use async_trait::async_trait;

use crate::{auth_error::AuthError, credentials::Credentials, user::UserId};

#[async_trait]
pub trait CredentialsRepository: Send + Sync {
    async fn find_by_user_id(&self, user_id: &UserId) -> Result<Option<Credentials>, AuthError>;
    async fn save(&self, credentials: &Credentials) -> Result<(), AuthError>;
}

#[async_trait]
impl<T: CredentialsRepository + ?Sized> CredentialsRepository for Arc<T> {
    async fn find_by_user_id(&self, user_id: &UserId) -> Result<Option<Credentials>, AuthError> {
        (**self).find_by_user_id(user_id).await
    }

    async fn save(&self, credentials: &Credentials) -> Result<(), AuthError> {
        (**self).save(credentials).await
    }
}
