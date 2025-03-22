use crate::models::user::SlimUser;
use actix_identity::Identity;
use actix_web::{get, web, HttpResponse};

#[get("/me")]
pub async fn get_me(user: Option<Identity>) -> HttpResponse {
    if let Some(identity) = user {
        if let Ok(user_json) = identity.id() {
            println!("{:?}", user_json);
            if let Ok(user) = serde_json::from_str::<SlimUser>(&user_json) {
                return HttpResponse::Ok().json(user);
            }
        }
    }
    HttpResponse::Unauthorized().finish()
}

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(get_me);
}
