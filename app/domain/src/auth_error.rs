#[derive(Debug)]
pub enum AuthError {
    InvalidCredentials,
    UserNotFound,
    Unexpected(String),
}

impl std::fmt::Display for AuthError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AuthError::InvalidCredentials => write!(f, "invalid credentials"),
            AuthError::UserNotFound => write!(f, "user not found"),
            AuthError::Unexpected(msg) => write!(f, "unexpected error: {msg}"),
        }
    }
}

impl std::error::Error for AuthError {}
