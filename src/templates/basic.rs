use actix_identity::Identity;
use actix_web::{HttpResponse, Responder, get};
use tera::Context;
use crate::{models::user::SlimUser, templates::TEMPLATES};

#[get("/login-page")]
pub async fn render_login_page() -> HttpResponse {
    let context = Context::new();
    match TEMPLATES.render("auth/login.html", &context) {
        Ok(content) => HttpResponse::Ok().content_type("text/html").body(content),
        Err(err) => {
            eprintln!("Template error: {}", err);
            HttpResponse::InternalServerError().body("Template error")
        }
    }
}

#[get("/signup-page")]
pub async fn render_signup_page() -> impl Responder {
    let context = Context::new();
    match TEMPLATES.render("auth/signup.html", &context) {
        Ok(content) => HttpResponse::Ok().content_type("text/html").body(content),
        Err(err) => {
            eprintln!("Template error: {}", err);
            HttpResponse::InternalServerError().body("Template error")
        }
    }
}

#[get("/profile-page")]
pub async fn render_profile_page(identity: Option<Identity>) -> HttpResponse {
    render_profile(identity)
}

pub fn render_profile(identity: Option<Identity>) -> HttpResponse {
    let mut context = Context::new();
    
    if let Some(id) = identity {
        if let Ok(user_json) = id.id() {
            if let Ok(user) = serde_json::from_str::<SlimUser>(&user_json) {
                context.insert("user", &user);
                context.insert("is_authenticated", &true);
            } else {
                context.insert("is_authenticated", &false);
            }
        } else {
            context.insert("is_authenticated", &false);
        }
    } else {
        context.insert("is_authenticated", &false);
    }
    
    match TEMPLATES.render("profile.html", &context) {
        Ok(content) => HttpResponse::Ok().content_type("text/html").body(content),
        Err(err) => {
            eprintln!("Template error: {}", err);
            HttpResponse::InternalServerError().body("Template error")
        }
    }
}