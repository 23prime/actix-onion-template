use chrono::Utc;
use domain::{
    auth_error::AuthError,
    credentials::Credentials,
    credentials_repository::CredentialsRepository,
    user::{User, UserError, UserId, UserRepository},
};
use garde::Validate;
use uuid::Uuid;

#[derive(Validate)]
pub struct CreateUserInput {
    #[garde(length(min = 1))]
    pub name: String,
    #[garde(email)]
    pub email: String,
    #[garde(length(min = 8))]
    pub password: String,
}

impl std::fmt::Debug for CreateUserInput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CreateUserInput")
            .field("name", &self.name)
            .field("email", &self.email)
            .field("password", &"[REDACTED]")
            .finish()
    }
}

pub struct CreateUser<R: UserRepository, C: CredentialsRepository> {
    user_repo: R,
    credentials_repo: C,
}

impl<R: UserRepository, C: CredentialsRepository> CreateUser<R, C> {
    pub fn new(user_repo: R, credentials_repo: C) -> Self {
        Self {
            user_repo,
            credentials_repo,
        }
    }

    pub async fn execute(&self, input: CreateUserInput) -> Result<User, CreateUserError> {
        input.validate().map_err(CreateUserError::Validation)?;
        let user = User {
            id: UserId::new(Uuid::now_v7()),
            name: input.name,
            email: input.email,
            created_at: Utc::now(),
        };
        self.user_repo
            .save(&user)
            .await
            .map_err(CreateUserError::User)?;

        let credentials =
            Credentials::new(user.id.clone(), &input.password).map_err(CreateUserError::Auth)?;
        self.credentials_repo
            .save(&credentials)
            .await
            .map_err(CreateUserError::Auth)?;

        Ok(user)
    }
}

#[derive(Debug)]
pub enum CreateUserError {
    Validation(garde::Report),
    User(UserError),
    Auth(AuthError),
}

impl std::fmt::Display for CreateUserError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CreateUserError::Validation(report) => write!(f, "validation error: {report}"),
            CreateUserError::User(e) => write!(f, "{e}"),
            CreateUserError::Auth(e) => write!(f, "{e}"),
        }
    }
}

impl std::error::Error for CreateUserError {}

#[cfg(test)]
mod tests {
    use garde::Validate;

    use super::*;

    fn valid_input() -> CreateUserInput {
        CreateUserInput {
            name: "Alice".to_string(),
            email: "alice@example.com".to_string(),
            password: "password123".to_string(),
        }
    }

    #[test]
    fn valid_input_passes() {
        assert!(valid_input().validate().is_ok());
    }

    #[test]
    fn empty_name_is_rejected() {
        let input = CreateUserInput {
            name: "".to_string(),
            ..valid_input()
        };
        let report = input.validate().unwrap_err();
        assert!(report.iter().any(|(path, _)| path.to_string() == "name"));
    }

    #[test]
    fn invalid_email_is_rejected() {
        let input = CreateUserInput {
            email: "not-an-email".to_string(),
            ..valid_input()
        };
        let report = input.validate().unwrap_err();
        assert!(report.iter().any(|(path, _)| path.to_string() == "email"));
    }

    #[test]
    fn short_password_is_rejected() {
        let input = CreateUserInput {
            password: "short".to_string(),
            ..valid_input()
        };
        let report = input.validate().unwrap_err();
        assert!(
            report
                .iter()
                .any(|(path, _)| path.to_string() == "password")
        );
    }

    #[test]
    fn multiple_errors_are_reported_together() {
        let input = CreateUserInput {
            name: "".to_string(),
            email: "bad".to_string(),
            ..valid_input()
        };
        let report = input.validate().unwrap_err();
        let paths: Vec<_> = report.iter().map(|(path, _)| path.to_string()).collect();
        assert!(paths.contains(&"name".to_string()));
        assert!(paths.contains(&"email".to_string()));
    }
}
