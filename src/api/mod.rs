use actix_web::web;

pub mod auth;
pub mod user;

pub fn configure_api(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/v1")
            .service(
                web::scope("/auth")
                    .configure(auth::configure)
            )
            .service(
                web::scope("/users")
                    .configure(user::configure)
            )
    );
}
