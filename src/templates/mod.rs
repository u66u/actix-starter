use actix_web::web;
use lazy_static::lazy_static;
use tera::Tera;

mod user;

lazy_static! {
    pub static ref TEMPLATES: Tera = {
        let source = "templates/**/*";
        let tera = Tera::new(source).unwrap();
        tera
    };
}

pub fn configure_templates(cfg: &mut web::ServiceConfig) {
    cfg.service(user::render_login_page)
        .service(user::render_signup_page)
        .service(user::render_profile_page);
}
