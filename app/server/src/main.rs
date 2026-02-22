mod config;

use actix_web::{App, HttpServer};
use tracing_actix_web::TracingLogger;
use tracing_subscriber::{
    EnvFilter, Layer, fmt::format::FmtSpan, layer::SubscriberExt, util::SubscriberInitExt,
};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let config = config::Config::from_env().unwrap_or_else(|e| {
        eprintln!("Configuration error: {e}");
        std::process::exit(1);
    });

    let fmt_layer: Box<dyn Layer<_> + Send + Sync> = match config.log_format.as_str() {
        "text" => tracing_subscriber::fmt::layer()
            .with_span_events(FmtSpan::NEW | FmtSpan::CLOSE)
            .boxed(),
        _ => tracing_subscriber::fmt::layer()
            .json()
            .with_span_events(FmtSpan::NEW | FmtSpan::CLOSE)
            .boxed(),
    };

    tracing_subscriber::registry()
        .with(EnvFilter::new(&config.log_level))
        .with(fmt_layer)
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
