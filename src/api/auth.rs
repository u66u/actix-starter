use chrono::NaiveDateTime;
use std::future::{ready, Ready};
use std::time::Instant;
use uuid::Uuid;

use actix_identity::Identity;
use actix_web::{
    dev::Payload, get, post, web, Error, FromRequest, HttpMessage, HttpRequest, HttpResponse,
};
use sqlx::SqlitePool;

use crate::errors::ServiceError;
use crate::models::user::{CreateUser, LoginUser, SlimUser, User};
use crate::utils::password::{hash_password, verify};

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

#[post("/register")]
pub async fn register(
    pool: web::Data<SqlitePool>,
    user_data: web::Json<CreateUser>,
) -> Result<HttpResponse, ServiceError> {
    let hashed_password = hash_password(&user_data.password)?;
    let user_id = Uuid::new_v4();

    let user = sqlx::query_as!(
        User,
        r#"
        INSERT INTO users (id, email, hash)
        VALUES (?1, ?2, ?3)
        RETURNING id as "id: uuid::Uuid", email, hash, created_at
        "#,
        user_id,
        user_data.email,
        hashed_password
    )
    .fetch_one(pool.get_ref())
    .await
    .map_err(|e| {
        if e.to_string().contains("UNIQUE constraint failed") {
            ServiceError::BadRequest("Email already exists".into())
        } else {
            ServiceError::InternalServerError
        }
    })?;

    Ok(HttpResponse::Ok().json(SlimUser::from(user)))
}

#[post("/login")]
pub async fn login(
    req: HttpRequest,
    user_data: web::Json<LoginUser>,
    pool: web::Data<SqlitePool>,
) -> Result<HttpResponse, ServiceError> {
    let user = sqlx::query_as!(
        User,
        r#"
        SELECT 
            id as "id: Uuid",
            email,
            hash,
            created_at
        FROM users 
        WHERE email = ?
        "#,
        user_data.email
    )
    .fetch_optional(pool.get_ref())
    .await
    .map_err(|_| ServiceError::InternalServerError)?
    .ok_or(ServiceError::Unauthorized)?;

    let is_valid = verify(&user.hash, &user_data.password)?;

    if is_valid {
        let slim_user = SlimUser::from(user);
        println!("logged in: {:?}", &slim_user);
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
pub async fn logout(identity: Identity) -> HttpResponse {
    identity.logout();
    HttpResponse::Ok().finish()
}

#[get("/me")]
pub async fn get_me(user: Option<Identity>) -> HttpResponse {
    if let Some(identity) = user {
        if let Ok(user_json) = identity.id() {
            if let Ok(user) = serde_json::from_str::<SlimUser>(&user_json) {
                return HttpResponse::Ok().json(user);
            }
        }
    }
    HttpResponse::Unauthorized().finish()
}
