use domain::{auth_error::AuthError, user::UserId};
use jsonwebtoken::{EncodingKey, Header, encode};
use serde::{Deserialize, Serialize};

pub struct JwtConfig {
    pub secret: String,
    pub expires_in_secs: u64,
}

#[derive(Serialize, Deserialize)]
struct Claims {
    sub: String,
    iat: u64,
    exp: u64,
}

pub fn issue_token(user_id: &UserId, config: &JwtConfig) -> Result<String, AuthError> {
    let now = jsonwebtoken::get_current_timestamp();
    let claims = Claims {
        sub: user_id.0.to_string(),
        iat: now,
        exp: now + config.expires_in_secs,
    };
    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(config.secret.as_bytes()),
    )
    .map_err(|e| AuthError::Unexpected(e.to_string()))
}
