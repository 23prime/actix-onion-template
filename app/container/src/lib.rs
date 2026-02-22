use std::sync::Arc;

use domain::user::UserRepository;

pub struct Container {
    pub user_repo: Arc<dyn UserRepository>,
}

impl Container {
    pub fn new(user_repo: Arc<dyn UserRepository>) -> Self {
        Self { user_repo }
    }
}
