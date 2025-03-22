use actix_web::web;
use tera::Tera;
use lazy_static::lazy_static;


mod basic;

lazy_static! {
    pub static ref TEMPLATES: Tera = {
        let source = "templates/**/*";
        let tera = Tera::new(source).unwrap();
        tera
    };
}

pub fn configure_templates(cfg: &mut web::ServiceConfig) {
    cfg.service(basic::render_login_page)
       .service(basic::render_signup_page)
       .service(basic::render_profile_page);
}