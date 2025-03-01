mod db;
mod models;
mod api;
mod utils;
mod errors;

use actix_identity::{IdentityMiddleware, Identity};
use actix_session::{storage::CookieSessionStore, SessionMiddleware};
use actix_web::{cookie::Key, web, App, HttpServer, middleware};
use api::auth::{login, logout, register, get_me};
use std::time::Duration;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let pool = db::create_pool()
        .await
        .expect("Failed to create database pool");

    // Run migrations
    sqlx::migrate!("./migrations")
        .run(&pool)
        .await
        .expect("Failed to run migrations");

    let secret_key = Key::generate();

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .wrap(IdentityMiddleware::default())
            .wrap(
                SessionMiddleware::builder(
                    CookieSessionStore::default(),
                    secret_key.clone(),
                )
                .cookie_secure(false)  // set to true in production with HTTPS
                .build(),
            )
            .wrap(middleware::Logger::default())
            .service(register)
            .service(login)
            .service(logout)
            .service(get_me)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}