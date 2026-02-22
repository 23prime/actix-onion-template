use chrono::Utc;
use domain::user::{User, UserError, UserId, UserRepository};
use uuid::Uuid;

pub struct CreateUserInput {
    pub name: String,
    pub email: String,
}

pub struct CreateUser<R: UserRepository> {
    repo: R,
}

impl<R: UserRepository> CreateUser<R> {
    pub fn new(repo: R) -> Self {
        Self { repo }
    }

    pub async fn execute(&self, input: CreateUserInput) -> Result<User, UserError> {
        let user = User {
            id: UserId::new(Uuid::now_v7()),
            name: input.name,
            email: input.email,
            created_at: Utc::now(),
        };
        self.repo.save(&user).await?;
        Ok(user)
    }
}

pub struct GetUser<R: UserRepository> {
    repo: R,
}

impl<R: UserRepository> GetUser<R> {
    pub fn new(repo: R) -> Self {
        Self { repo }
    }

    pub async fn execute(&self, id: UserId) -> Result<User, UserError> {
        self.repo.find_by_id(&id).await?.ok_or(UserError::NotFound)
    }
}
