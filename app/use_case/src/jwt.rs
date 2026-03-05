use domain::user::UserId;
use jsonwebtoken::{EncodingKey, Header, encode};
use serde::{Deserialize, Serialize};

pub struct JwtConfig {
    pub secret: String,
    pub expires_in_secs: u64,
}

#[derive(Debug)]
pub struct JwtError(String);

impl std::fmt::Display for JwtError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "jwt error: {}", self.0)
    }
}

impl std::error::Error for JwtError {}

#[derive(Serialize, Deserialize)]
struct Claims {
    sub: String,
    iat: u64,
    exp: u64,
}

pub fn issue_token(user_id: &UserId, config: &JwtConfig) -> Result<String, JwtError> {
    let now = jsonwebtoken::get_current_timestamp();
    let exp = now
        .checked_add(config.expires_in_secs)
        .ok_or_else(|| JwtError("expires_in_secs overflow".to_string()))?;
    let claims = Claims {
        sub: user_id.0.to_string(),
        iat: now,
        exp,
    };
    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(config.secret.as_bytes()),
    )
    .map_err(|e| JwtError(e.to_string()))
}
