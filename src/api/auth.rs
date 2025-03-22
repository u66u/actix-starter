use std::future::{ready, Ready};

use actix_identity::Identity;
use actix_web::{
    dev::Payload, get, post, web, Error, FromRequest, HttpMessage, HttpRequest, HttpResponse,
    ResponseError,
};
use sqlx::SqlitePool;

use crate::db::{create_user, find_user_by_email};
use crate::errors::ServiceError;
use crate::models::user::{CreateUser, LoginUser, SlimUser, User};
use crate::utils::password::verify;

pub type LoggedUser = SlimUser;

impl FromRequest for LoggedUser {
    type Error = Error;
    type Future = Ready<Result<LoggedUser, Error>>;

    fn from_request(req: &HttpRequest, pl: &mut Payload) -> Self::Future {
        if let Ok(identity) = Identity::from_request(req, pl).into_inner() {
            if let Ok(user_json) = identity.id() {
                if let Ok(user) = serde_json::from_str(&user_json) {
                    return ready(Ok(user));
                }
            }
        }
        ready(Err(ServiceError::Unauthorized.into()))
    }
}

#[post("/signup")]
pub async fn signup(
    pool: web::Data<SqlitePool>,
    user_data: web::Json<CreateUser>,
) -> Result<HttpResponse, ServiceError> {
    let user = create_user(pool.get_ref(), &user_data).await?;
    Ok(HttpResponse::Ok().json(SlimUser::from(user)))
}

#[post("/login")]
pub async fn login(
    req: HttpRequest,
    user_data: web::Json<LoginUser>,
    pool: web::Data<SqlitePool>,
) -> Result<HttpResponse, ServiceError> {
    let user = find_user_by_email(pool.get_ref(), &user_data.email)
        .await?
        .ok_or(ServiceError::Unauthorized)?;

    let is_valid = verify(&user.hashed_pwd, &user_data.password)?;

    if is_valid {
        let slim_user = SlimUser::from(user);
        let user_string =
            serde_json::to_string(&slim_user).map_err(|_| ServiceError::InternalServerError)?;
        Identity::login(&req.extensions(), user_string)
            .map_err(|_| ServiceError::InternalServerError)?;

        Ok(HttpResponse::Ok().json(slim_user))
    } else {
        Err(ServiceError::Unauthorized)
    }
}

#[post("/logout")]
pub async fn logout(user: Option<Identity>) -> HttpResponse {
    if let Some(id) = user {
        id.logout();
        HttpResponse::Ok().finish()
    } else {
        HttpResponse::Unauthorized().finish()
    }
}

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(signup).service(login).service(logout);
}
