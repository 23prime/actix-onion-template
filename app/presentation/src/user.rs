use actix_web::{HttpResponse, web};
use domain::user::UserId;
use infrastructure::user_repository::PgUserRepository;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use use_case::user::{CreateUser, CreateUserInput, GetUser};
use uuid::Uuid;

#[derive(Deserialize)]
pub struct CreateUserRequest {
    pub name: String,
    pub email: String,
}

#[derive(Serialize)]
pub struct UserResponse {
    pub id: String,
    pub name: String,
    pub email: String,
    pub created_at: String,
}

pub async fn create_user(
    pool: web::Data<PgPool>,
    body: web::Json<CreateUserRequest>,
) -> HttpResponse {
    let repo = PgUserRepository::new(pool.get_ref().clone());
    let use_case = CreateUser::new(repo);
    let input = CreateUserInput {
        name: body.name.clone(),
        email: body.email.clone(),
    };
    match use_case.execute(input).await {
        Ok(user) => HttpResponse::Created().json(UserResponse {
            id: user.id.0.to_string(),
            name: user.name,
            email: user.email,
            created_at: user.created_at.to_rfc3339(),
        }),
        Err(domain::user::UserError::EmailAlreadyExists) => {
            HttpResponse::Conflict().json(serde_json::json!({ "error": "email_already_exists" }))
        }
        Err(e) => {
            tracing::error!("create_user error: {e}");
            HttpResponse::InternalServerError()
                .json(serde_json::json!({ "error": "internal_server_error" }))
        }
    }
}

pub async fn get_user(pool: web::Data<PgPool>, path: web::Path<Uuid>) -> HttpResponse {
    let id = UserId::new(path.into_inner());
    let repo = PgUserRepository::new(pool.get_ref().clone());
    let use_case = GetUser::new(repo);
    match use_case.execute(id).await {
        Ok(user) => HttpResponse::Ok().json(UserResponse {
            id: user.id.0.to_string(),
            name: user.name,
            email: user.email,
            created_at: user.created_at.to_rfc3339(),
        }),
        Err(domain::user::UserError::NotFound) => {
            HttpResponse::NotFound().json(serde_json::json!({ "error": "not_found" }))
        }
        Err(e) => {
            tracing::error!("get_user error: {e}");
            HttpResponse::InternalServerError()
                .json(serde_json::json!({ "error": "internal_server_error" }))
        }
    }
}

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/users")
            .route("", web::post().to(create_user))
            .route("/{id}", web::get().to(get_user)),
    );
}
