use domain::{credentials_repository::CredentialsRepository, user::UserRepository};
use garde::Validate;

use crate::jwt::{JwtConfig, issue_token};

#[derive(Validate)]
pub struct LoginInput {
    #[garde(email)]
    pub email: String,
    #[garde(length(min = 8))]
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

fn to_unexpected<E: std::fmt::Display>(e: E) -> LoginError {
    LoginError::Unexpected(e.to_string())
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
    ) -> Result<String, LoginError> {
        input.validate().map_err(LoginError::Validation)?;

        let user = self
            .user_repo
            .find_by_email(&input.email)
            .await
            .map_err(to_unexpected)?
            .ok_or(LoginError::InvalidCredentials)?;

        let credentials = self
            .credentials_repo
            .find_by_user_id(&user.id)
            .await
            .map_err(to_unexpected)?
            .ok_or(LoginError::InvalidCredentials)?;

        if !credentials.verify_password(&input.password) {
            return Err(LoginError::InvalidCredentials);
        }

        issue_token(&user.id, jwt_config).map_err(to_unexpected)
    }
}

#[derive(Debug)]
pub enum LoginError {
    Validation(garde::Report),
    InvalidCredentials,
    Unexpected(String),
}

impl std::fmt::Display for LoginError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LoginError::Validation(report) => write!(f, "validation error: {report}"),
            LoginError::InvalidCredentials => write!(f, "invalid credentials"),
            LoginError::Unexpected(msg) => write!(f, "unexpected error: {msg}"),
        }
    }
}

impl std::error::Error for LoginError {}

#[cfg(test)]
mod tests {
    use async_trait::async_trait;
    use domain::{
        auth_error::AuthError,
        credentials::Credentials,
        user::{User, UserError, UserId},
    };
    use garde::Validate;

    use super::*;

    struct PanicUserRepo;

    #[async_trait]
    impl domain::user::UserRepository for PanicUserRepo {
        async fn find_by_id(&self, _: &UserId) -> Result<Option<User>, UserError> {
            panic!("repo must not be called")
        }

        async fn find_by_email(&self, _: &str) -> Result<Option<User>, UserError> {
            panic!("repo must not be called")
        }

        async fn save(&self, _: &User) -> Result<(), UserError> {
            panic!("repo must not be called")
        }
    }

    struct PanicCredentialsRepo;

    #[async_trait]
    impl domain::credentials_repository::CredentialsRepository for PanicCredentialsRepo {
        async fn find_by_user_id(&self, _: &UserId) -> Result<Option<Credentials>, AuthError> {
            panic!("repo must not be called")
        }

        async fn save(&self, _: &Credentials) -> Result<(), AuthError> {
            panic!("repo must not be called")
        }
    }

    fn valid_input() -> LoginInput {
        LoginInput {
            email: "alice@example.com".to_string(),
            password: "password123".to_string(),
        }
    }

    #[test]
    fn valid_input_passes() {
        assert!(valid_input().validate().is_ok());
    }

    #[test]
    fn invalid_email_is_rejected() {
        let input = LoginInput {
            email: "bad".to_string(),
            ..valid_input()
        };
        let report = input.validate().unwrap_err();
        assert!(report.iter().any(|(path, _)| path.to_string() == "email"));
    }

    #[test]
    fn short_password_is_rejected() {
        let input = LoginInput {
            password: "1234567".to_string(),
            ..valid_input()
        };
        let report = input.validate().unwrap_err();
        assert!(
            report
                .iter()
                .any(|(path, _)| path.to_string() == "password")
        );
    }

    #[tokio::test]
    async fn execute_returns_validation_error_before_calling_repo() {
        let use_case = Login::new(PanicUserRepo, PanicCredentialsRepo);
        let input = LoginInput {
            email: "not-an-email".to_string(),
            password: "short".to_string(),
        };
        let jwt_config = crate::jwt::JwtConfig {
            secret: "secret".to_string(),
            expires_in_secs: 3600,
        };
        let result = use_case.execute(input, &jwt_config).await;
        assert!(matches!(result, Err(LoginError::Validation(_))));
    }
}
