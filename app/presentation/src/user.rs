use actix_web::{HttpResponse, web};
use container::Container;
use domain::user::UserId;
use serde::{Deserialize, Serialize};
use use_case::{
    create_user::{CreateUser, CreateUserInput},
    get_user::GetUser,
};
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
    container: web::Data<Container>,
    body: web::Json<CreateUserRequest>,
) -> HttpResponse {
    let use_case = CreateUser::new(container.user_repo.clone());
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

pub async fn get_user(container: web::Data<Container>, path: web::Path<Uuid>) -> HttpResponse {
    let id = UserId::new(path.into_inner());
    let use_case = GetUser::new(container.user_repo.clone());
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
