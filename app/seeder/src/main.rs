mod local;

#[tokio::main]
async fn main() {
    let env = std::env::var("ENV").unwrap_or_else(|_| "local".into());
    let database_url = std::env::var("DATABASE_URL").unwrap_or_else(|_| {
        eprintln!("DATABASE_URL must be set");
        std::process::exit(1);
    });

    let pool = sqlx::PgPool::connect(&database_url)
        .await
        .unwrap_or_else(|e| {
            eprintln!("Database connection error: {e}");
            std::process::exit(1);
        });

    match env.as_str() {
        "local" => local::seed(&pool).await,
        _ => {
            eprintln!("Unknown environment: {env}");
            std::process::exit(1);
        }
    }
}
