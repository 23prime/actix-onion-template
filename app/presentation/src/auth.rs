use actix_web::{HttpResponse, web};
use container::Container;
use serde::{Deserialize, Serialize};
use use_case::{
    LoginError,
    jwt::JwtConfig,
    login::{Login, LoginInput},
};

use crate::validation_fields;

#[derive(Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

#[derive(Serialize)]
pub struct TokenResponse {
    pub access_token: String,
    pub token_type: String,
    pub expires_in: u64,
}

pub async fn login(
    container: web::Data<Container>,
    jwt_config: web::Data<JwtConfig>,
    body: web::Json<LoginRequest>,
) -> HttpResponse {
    let use_case = Login::new(
        container.user_repo.clone(),
        container.credentials_repo.clone(),
    );
    let input = LoginInput {
        email: body.email.clone(),
        password: body.password.clone(),
    };
    match use_case.execute(input, &jwt_config).await {
        Ok(token) => HttpResponse::Ok().json(TokenResponse {
            access_token: token,
            token_type: "Bearer".to_string(),
            expires_in: jwt_config.expires_in_secs,
        }),
        Err(LoginError::Validation(report)) => {
            HttpResponse::UnprocessableEntity().json(serde_json::json!({
                "error": "validation_error",
                "fields": validation_fields(&report),
            }))
        }
        Err(LoginError::InvalidCredentials) => {
            HttpResponse::Unauthorized().json(serde_json::json!({ "error": "invalid_credentials" }))
        }
        Err(e) => {
            tracing::error!("login error: {e}");
            HttpResponse::InternalServerError()
                .json(serde_json::json!({ "error": "internal_server_error" }))
        }
    }
}

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/auth").route("/login", web::post().to(login)));
}
