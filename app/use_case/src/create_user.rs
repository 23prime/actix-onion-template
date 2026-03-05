use chrono::Utc;
use domain::user::{Credential, PasswordCredential, User, UserError, UserId, UserRepository};
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

pub struct CreateUser<R: UserRepository> {
    user_repo: R,
}

impl<R: UserRepository> CreateUser<R> {
    pub fn new(user_repo: R) -> Self {
        Self { user_repo }
    }

    pub async fn execute(&self, input: CreateUserInput) -> Result<User, CreateUserError> {
        input.validate().map_err(CreateUserError::Validation)?;

        let credential = PasswordCredential::new(&input.password).map_err(CreateUserError::User)?;
        let user = User {
            id: UserId::new(Uuid::now_v7()),
            name: input.name,
            email: input.email,
            created_at: Utc::now(),
            credential: Credential::Password(credential),
        };

        self.user_repo
            .save(&user)
            .await
            .map_err(CreateUserError::User)?;

        Ok(user)
    }
}

#[derive(Debug)]
pub enum CreateUserError {
    Validation(garde::Report),
    User(UserError),
}

impl std::fmt::Display for CreateUserError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CreateUserError::Validation(report) => write!(f, "validation error: {report}"),
            CreateUserError::User(e) => write!(f, "{e}"),
        }
    }
}

impl std::error::Error for CreateUserError {}

#[cfg(test)]
mod tests {
    use async_trait::async_trait;
    use domain::user::{User, UserError, UserId};
    use garde::Validate;
    use std::sync::Mutex;

    use super::*;

    struct CapturingUserRepo {
        saved: Mutex<Vec<User>>,
    }

    impl CapturingUserRepo {
        fn new() -> Self {
            Self {
                saved: Mutex::new(vec![]),
            }
        }
    }

    #[async_trait]
    impl UserRepository for CapturingUserRepo {
        async fn find_by_id(&self, _: &UserId) -> Result<Option<User>, UserError> {
            panic!("not expected")
        }

        async fn find_by_email(&self, _: &str) -> Result<Option<User>, UserError> {
            panic!("not expected")
        }

        async fn save(&self, user: &User) -> Result<(), UserError> {
            self.saved.lock().unwrap().push(user.clone());
            Ok(())
        }
    }

    #[tokio::test]
    async fn execute_attaches_credential_and_password_verifies() {
        let repo = CapturingUserRepo::new();
        let use_case = CreateUser::new(repo);
        let password = "password123".to_string();
        let input = CreateUserInput {
            name: "Alice".to_string(),
            email: "alice@example.com".to_string(),
            password: password.clone(),
        };

        let user = use_case.execute(input).await.unwrap();

        assert!(user.verify_password(&password));
        assert!(!user.verify_password("wrong-password"));
    }

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
