use domain::{
    auth_error::AuthError, credentials_repository::CredentialsRepository, user::UserRepository,
};

use crate::jwt::{JwtConfig, issue_token};

pub struct LoginInput {
    pub email: String,
    pub password: String,
}

impl std::fmt::Debug for LoginInput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("LoginInput")
            .field("email", &self.email)
            .field("password", &"[REDACTED]")
            .finish()
    }
}

pub struct Login<R: UserRepository, C: CredentialsRepository> {
    user_repo: R,
    credentials_repo: C,
}

impl<R: UserRepository, C: CredentialsRepository> Login<R, C> {
    pub fn new(user_repo: R, credentials_repo: C) -> Self {
        Self {
            user_repo,
            credentials_repo,
        }
    }

    pub async fn execute(
        &self,
        input: LoginInput,
        jwt_config: &JwtConfig,
    ) -> Result<String, AuthError> {
        let user = self
            .user_repo
            .find_by_email(&input.email)
            .await
            .map_err(|e| AuthError::Unexpected(e.to_string()))?
            .ok_or(AuthError::InvalidCredentials)?;

        let credentials = self
            .credentials_repo
            .find_by_user_id(&user.id)
            .await?
            .ok_or(AuthError::InvalidCredentials)?;

        if !credentials.verify_password(&input.password) {
            return Err(AuthError::InvalidCredentials);
        }

        issue_token(&user.id, jwt_config)
    }
}
