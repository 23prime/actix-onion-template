mod config;

use actix_web::{App, HttpServer};
use tracing_actix_web::TracingLogger;
use tracing_subscriber::{EnvFilter, layer::SubscriberExt, util::SubscriberInitExt};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let config = config::Config::from_env().unwrap_or_else(|e| {
        eprintln!("Configuration error: {e}");
        std::process::exit(1);
    });

    tracing_subscriber::registry()
        .with(EnvFilter::new(&config.log_level))
        .with(tracing_subscriber::fmt::layer().json())
        .init();

    tracing::info!(port = config.port, "Starting server");

    HttpServer::new(|| {
        App::new()
            .wrap(TracingLogger::default())
            .configure(presentation::configure)
    })
    .bind(("0.0.0.0", config.port))?
    .run()
    .await
}
