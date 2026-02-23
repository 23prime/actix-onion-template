use chrono::Utc;
use domain::{
    credentials::Credentials,
    credentials_repository::CredentialsRepository,
    user::{User, UserError, UserId, UserRepository},
};
use infrastructure::{
    credentials_repository::PgCredentialsRepository, user_repository::PgUserRepository,
};
use sqlx::PgPool;
use uuid::Uuid;

pub async fn seed(pool: &PgPool) {
    println!("Seeding local database...");
    let user_repo = PgUserRepository::new(pool.clone());
    let credentials_repo = PgCredentialsRepository::new(pool.clone());
    create_user(
        &user_repo,
        &credentials_repo,
        "Admin",
        "admin@example.com",
        "admin",
    )
    .await;
    println!("Done.");
}

async fn create_user(
    user_repo: &PgUserRepository,
    credentials_repo: &PgCredentialsRepository,
    name: &str,
    email: &str,
    password: &str,
) {
    let user = User {
        id: UserId::new(Uuid::now_v7()),
        name: name.to_string(),
        email: email.to_string(),
        created_at: Utc::now(),
    };

    match user_repo.save(&user).await {
        Ok(()) => {}
        Err(UserError::EmailAlreadyExists) => {
            println!("  skipped (already exists): {email}");
            return;
        }
        Err(e) => panic!("Failed to insert user {email}: {e}"),
    }

    let credentials = Credentials::new(user.id, password)
        .unwrap_or_else(|e| panic!("Failed to hash password for {email}: {e}"));

    credentials_repo
        .save(&credentials)
        .await
        .unwrap_or_else(|e| panic!("Failed to insert credentials for {email}: {e}"));

    println!("  created: {email}");
}
