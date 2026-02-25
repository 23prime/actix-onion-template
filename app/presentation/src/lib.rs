pub mod auth;
mod health;
pub mod user;

use actix_web::web;

pub(crate) fn validation_fields(
    report: &garde::Report,
) -> std::collections::HashMap<String, Vec<String>> {
    let mut fields: std::collections::HashMap<String, Vec<String>> =
        std::collections::HashMap::new();
    for (path, error) in report.iter() {
        fields
            .entry(path.to_string())
            .or_default()
            .push(error.message().to_string());
    }
    fields
}

pub fn configure(cfg: &mut web::ServiceConfig) {
    auth::configure(cfg);
    health::configure(cfg);
    user::configure(cfg);
}
