mod config;

use actix_web::{App, HttpServer};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let config = config::Config::from_env().unwrap_or_else(|e| {
        eprintln!("Configuration error: {e}");
        std::process::exit(1);
    });

    HttpServer::new(|| App::new().configure(presentation::configure))
        .bind(("0.0.0.0", config.port))?
        .run()
        .await
}
