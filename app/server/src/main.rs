mod config;
mod middleware;
mod tracing;

use std::sync::Arc;

use actix_web::{App, HttpServer, web::Data};
use container::Container;
use infrastructure::user_repository::PgUserRepository;
use tracing_actix_web::TracingLogger;
use use_case::jwt::JwtConfig;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let config = config::Config::from_env().unwrap_or_else(|e| {
        eprintln!("Configuration error: {e}");
        std::process::exit(1);
    });

    tracing::init(&config);

    ::tracing::info!(port = config.port, "Starting server");

    let pool = sqlx::PgPool::connect(&config.database_url)
        .await
        .unwrap_or_else(|e| {
            eprintln!("Database connection error: {e}");
            std::process::exit(1);
        });

    let container = Data::new(Container::new(Arc::new(PgUserRepository::new(pool))));

    let jwt_config = Data::new(JwtConfig {
        secret: config.jwt_secret.clone(),
        expires_in_secs: config.jwt_expires_in_secs,
    });

    HttpServer::new(move || {
        App::new()
            .app_data(container.clone())
            .app_data(jwt_config.clone())
            .wrap(middleware::cors(&config.cors_allowed_origins))
            .wrap(middleware::default_headers())
            .wrap(TracingLogger::default())
            .configure(presentation::configure)
    })
    .bind(("0.0.0.0", config.port))?
    .run()
    .await
}
