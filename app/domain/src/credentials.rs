use argon2::{
    Argon2,
    password_hash::{PasswordHash, PasswordHasher, PasswordVerifier, SaltString, rand_core::OsRng},
};
use chrono::{DateTime, Utc};

use crate::{auth_error::AuthError, user::UserId};

pub struct Credentials {
    pub user_id: UserId,
    pub password_hash: String,
    pub created_at: DateTime<Utc>,
}

impl Credentials {
    pub fn new(user_id: UserId, password: &str) -> Result<Self, AuthError> {
        let salt = SaltString::generate(&mut OsRng);
        let argon2 = Argon2::default();
        let password_hash = argon2
            .hash_password(password.as_bytes(), &salt)
            .map_err(|e| AuthError::Unexpected(e.to_string()))?
            .to_string();

        Ok(Self {
            user_id,
            password_hash,
            created_at: Utc::now(),
        })
    }

    pub fn verify_password(&self, password: &str) -> bool {
        let Ok(parsed_hash) = PasswordHash::new(&self.password_hash) else {
            return false;
        };
        Argon2::default()
            .verify_password(password.as_bytes(), &parsed_hash)
            .is_ok()
    }
}
