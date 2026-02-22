mod config;
mod middleware;
mod tracing;

use actix_web::{App, HttpServer};
use tracing_actix_web::TracingLogger;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let config = config::Config::from_env().unwrap_or_else(|e| {
        eprintln!("Configuration error: {e}");
        std::process::exit(1);
    });

    tracing::init(&config);

    ::tracing::info!(port = config.port, "Starting server");

    HttpServer::new(move || {
        App::new()
            .wrap(middleware::cors(&config.cors_allowed_origins))
            .wrap(middleware::default_headers())
            .wrap(TracingLogger::default())
            .configure(presentation::configure)
    })
    .bind(("0.0.0.0", config.port))?
    .run()
    .await
}
