mod api;
mod db;
mod errors;
mod models;
mod templates;
mod utils;

use actix_files;
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
use env_logger::Env;
use utils::config::{self, CONFIG};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let app_cfg = CONFIG.clone();
    env_logger::init_from_env(Env::default().default_filter_or("info"));

    let redis_store = RedisSessionStore::new(app_cfg.redis_url.clone())
        .await
        .unwrap();

    let pool = db::create_pool()
        .await
        .expect("Failed to create database pool");

    sqlx::migrate!("./migrations")
        .run(&pool)
        .await
        .expect("Failed to run migrations");

    let secret_key = Key::from(app_cfg.secret_key.as_bytes());

    HttpServer::new(move || {
        let cookie_ttl = app_cfg.cookie_ttl_secs;

        let identity_middleware = IdentityMiddleware::builder()
            .visit_deadline(Some(cookie_ttl))
            .login_deadline(Some(cookie_ttl * 3))
            .logout_behaviour(LogoutBehaviour::PurgeSession)
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
            .service(web::scope("/api").configure(api::configure_api))
            .configure(templates::configure_templates)
            .service(actix_files::Files::new("/static", "./static").show_files_listing())
    })
    .bind((app_cfg.server_host.clone(), app_cfg.server_port))?
    .workers(4)
    .run()
    .await
}
