use actix_web::web;

mod auth;
mod user;

pub fn configure_api(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/v1")
            .service(web::scope("/auth").configure(auth::configure))
            .service(web::scope("/user").configure(user::configure)),
    );
}
