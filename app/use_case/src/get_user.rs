use domain::user::{User, UserError, UserId, UserRepository};

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
