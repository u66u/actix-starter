mod api;
mod db;
mod errors;
mod models;
mod pages;
mod utils;

use crate::pages::*;
use actix_identity::{
    config::{IdentityMiddlewareBuilder, LogoutBehaviour},
    Identity, IdentityMiddleware,
};
use actix_session::{
    config::PersistentSession,
    storage::{CookieSessionStore, RedisSessionStore},
    SessionMiddleware,
};
use actix_web::{cookie::Key, middleware, web, App, HttpServer};
use api::auth::{get_me, login, logout, register};
use std::time::Duration;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let redis_store = RedisSessionStore::new("redis://127.0.0.1:6379")
        .await
        .unwrap();

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
        let identity_middleware = IdentityMiddleware::builder()
            .visit_deadline(Some(Duration::from_secs(172800))) // 2 days
            .login_deadline(Some(Duration::from_secs(172800 * 3))) // 6 days
            .logout_behaviour(LogoutBehaviour::PurgeSession)
            .build();

        let cookie_ttl = Duration::from_secs(24 * 60 * 60);

        let identity_mw = IdentityMiddleware::builder()
            .visit_deadline(Some(cookie_ttl))
            .build();

        let session_middleware =
            SessionMiddleware::builder(redis_store.clone(), secret_key.clone())
                // disable secure cookie for local testing
                .cookie_secure(false)
                // Set a ttl for the cookie if the identity should live longer than the user session
                .session_lifecycle(
                    PersistentSession::default().session_ttl(cookie_ttl.try_into().unwrap()),
                )
                .build();
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .wrap(identity_middleware)
            .wrap(session_middleware)
            .wrap(middleware::Logger::default())
            .service(register)
            .service(login)
            .service(logout)
            .service(get_me)
            .service(login_page)
            .service(signup_page)
            .service(profile_page)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
