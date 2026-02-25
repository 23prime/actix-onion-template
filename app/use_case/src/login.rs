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
            .map_err(|e| LoginError::Unexpected(e.to_string()))?
            .ok_or(LoginError::InvalidCredentials)?;

        let credentials = self
            .credentials_repo
            .find_by_user_id(&user.id)
            .await
            .map_err(|e| LoginError::Unexpected(e.to_string()))?
            .ok_or(LoginError::InvalidCredentials)?;

        if !credentials.verify_password(&input.password) {
            return Err(LoginError::InvalidCredentials);
        }

        issue_token(&user.id, jwt_config).map_err(|e| LoginError::Unexpected(e.to_string()))
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
    use garde::Validate;

    use super::*;

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
}
