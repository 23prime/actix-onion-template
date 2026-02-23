pub mod auth;
mod health;
pub mod user;

use actix_web::web;

pub fn configure(cfg: &mut web::ServiceConfig) {
    auth::configure(cfg);
    health::configure(cfg);
    user::configure(cfg);
}
