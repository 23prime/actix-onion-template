use std::sync::Arc;

use domain::{credentials_repository::CredentialsRepository, user::UserRepository};

pub struct Container {
    pub user_repo: Arc<dyn UserRepository>,
    pub credentials_repo: Arc<dyn CredentialsRepository>,
}

impl Container {
    pub fn new(
        user_repo: Arc<dyn UserRepository>,
        credentials_repo: Arc<dyn CredentialsRepository>,
    ) -> Self {
        Self {
            user_repo,
            credentials_repo,
        }
    }
}
