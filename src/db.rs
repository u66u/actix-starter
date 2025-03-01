use sqlx::sqlite::{SqlitePool, SqlitePoolOptions};
use std::time::Duration;


pub async fn create_pool() -> Result<SqlitePool, sqlx::Error> {
    let database_url = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "sqlite:db.db?mode=rwc".to_string());

    SqlitePoolOptions::new()
        .max_connections(5)
        .acquire_timeout(Duration::from_secs(3))
        .connect(&database_url)
        .await
}