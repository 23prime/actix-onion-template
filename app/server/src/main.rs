mod config;
mod middleware;
mod tracing;

use actix_web::{App, HttpServer, web::Data};
use tracing_actix_web::TracingLogger;

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

    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(pool.clone()))
            .wrap(middleware::cors(&config.cors_allowed_origins))
            .wrap(middleware::default_headers())
            .wrap(TracingLogger::default())
            .configure(presentation::configure)
    })
    .bind(("0.0.0.0", config.port))?
    .run()
    .await
}
