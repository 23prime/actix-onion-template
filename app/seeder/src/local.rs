use chrono::Utc;
use domain::user::{Credential, PasswordCredential, User, UserError, UserId, UserRepository};
use infrastructure::user_repository::PgUserRepository;
use sqlx::PgPool;
use uuid::Uuid;

pub async fn seed(pool: &PgPool) {
    println!("Seeding local database...");
    let repo = PgUserRepository::new(pool.clone());
    create_user(&repo, "Admin", "admin@example.com", "admin").await;
    println!("Done.");
}

async fn create_user(repo: &PgUserRepository, name: &str, email: &str, password: &str) {
    let credential = PasswordCredential::new(password)
        .unwrap_or_else(|e| panic!("Failed to hash password for {email}: {e}"));
    let user = User {
        id: UserId::new(Uuid::now_v7()),
        name: name.to_string(),
        email: email.to_string(),
        created_at: Utc::now(),
        credentials: vec![Credential::Password(credential)],
    };

    match repo.save(&user).await {
        Ok(()) => println!("  created: {email}"),
        Err(UserError::EmailAlreadyExists) => println!("  skipped (already exists): {email}"),
        Err(e) => panic!("Failed to insert user {email}: {e}"),
    }
}
