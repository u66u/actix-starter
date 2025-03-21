use actix_web::{HttpResponse, Responder};
use tera::Context;
use crate::templates::TEMPLATES;

pub fn render_login_page() -> HttpResponse {
    let context = Context::new();
    match TEMPLATES.render("auth/login.html", &context) {
        Ok(content) => HttpResponse::Ok().content_type("text/html").body(content),
        Err(err) => {
            eprintln!("Template error: {}", err);
            HttpResponse::InternalServerError().body("Template error")
        }
    }
}

pub fn render_signup_page() -> impl Responder {
    let context = Context::new();
    match TEMPLATES.render("auth/signup.html", &context) {
        Ok(content) => HttpResponse::Ok().content_type("text/html").body(content),
        Err(err) => {
            eprintln!("Template error: {}", err);
            HttpResponse::InternalServerError().body("Template error")
        }
    }
}

pub fn render_profile_page() -> HttpResponse {
    let context = Context::new();
    match TEMPLATES.render("profile.html", &context) {
        Ok(content) => HttpResponse::Ok().content_type("text/html").body(content),
        Err(err) => {
            eprintln!("Template error: {}", err);
            HttpResponse::InternalServerError().body("Template error")
        }
    }
}
