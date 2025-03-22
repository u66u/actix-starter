use sqlx::sqlite::{SqlitePool, SqlitePoolOptions};
use sqlx::{PgPool, pool::PoolOptions};
use std::time::Duration;
use uuid::Uuid;
use rand;

use crate::errors::ServiceError;
use crate::models::user::{CreateUser, User, SlimUser, UserPrivilege};
use crate::utils::password::hash_password;

pub async fn create_pool() -> Result<SqlitePool, sqlx::Error> {
    let database_url =
        std::env::var("DATABASE_URL").unwrap_or_else(|_| "sqlite:db.db?mode=rwc".to_string());

    SqlitePoolOptions::new()
        .max_connections(5)
        .acquire_timeout(Duration::from_secs(3))
        .connect(&database_url)
        .await
}

pub async fn register_user(
    pool: &SqlitePool,
    user_data: &CreateUser,
) -> Result<User, ServiceError> {
    let hashed_password = hash_password(&user_data.password)?;
    let privilege = UserPrivilege::User;

    let user = sqlx::query_as!(
        User,
        r#"
        INSERT INTO users (email, name, hashed_pwd, privilege)
        VALUES (?1, ?2, ?3, ?4)
        RETURNING 
            id as "id: u64", 
            email, 
            name, 
            hashed_pwd, 
            privilege as "privilege: UserPrivilege",
            created_at, 
            updated_at
        "#,
        user_data.email,
        user_data.name,
        hashed_password,
        privilege
    )
    .fetch_one(pool)
    .await
    .map_err(|e| {
        println!("Database error during registration: {:?}", e);
        if e.to_string().contains("UNIQUE constraint failed") {
            ServiceError::BadRequest("Email already exists".into())
        } else {
            ServiceError::InternalServerError
        }
    })?;

    println!("User registered successfully: ID={}", user.id);
    Ok(user)
}

pub async fn find_user_by_email(
    pool: &SqlitePool,
    email: &str,
) -> Result<Option<User>, ServiceError> {
    let user = sqlx::query_as!(
        User,
        r#"
        SELECT 
            id as "id: u64",
            email,
            name,
            hashed_pwd,
            privilege as "privilege: UserPrivilege",
            created_at,
            updated_at
        FROM users 
        WHERE email = ?
        "#,
        email
    )
    .fetch_optional(pool)
    .await
    .map_err(|_| ServiceError::InternalServerError)?;

    if let Some(ref user) = user {
        println!("Retrieved ID: {}", user.id);
    }

    Ok(user)
}