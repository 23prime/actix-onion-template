use chrono::Utc;
use domain::{
    auth_error::AuthError,
    credentials::Credentials,
    credentials_repository::CredentialsRepository,
    user::{User, UserError, UserId, UserRepository},
};
use uuid::Uuid;

pub struct CreateUserInput {
    pub name: String,
    pub email: String,
    pub password: String,
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
    User(UserError),
    Auth(AuthError),
}

impl std::fmt::Display for CreateUserError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CreateUserError::User(e) => write!(f, "{e}"),
            CreateUserError::Auth(e) => write!(f, "{e}"),
        }
    }
}

impl std::error::Error for CreateUserError {}
