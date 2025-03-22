use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
#[sqlx(rename_all = "lowercase")]
pub enum UserPrivilege {
    User,
    Vip,
    Mod,
    Admin,
}

#[derive(Debug, Serialize, FromRow)]
pub struct User {
    pub id: u64,
    pub email: String,
    pub name: Option<String>,
    #[serde(skip_serializing)]
    pub hashed_pwd: String,
    pub privilege: UserPrivilege,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SlimUser {
    pub id: u64,
    pub email: String,
    pub name: Option<String>,
}

impl From<User> for SlimUser {
    fn from(user: User) -> Self {
        SlimUser {
            id: user.id,
            email: user.email,
            name: user.name,
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct CreateUser {
    pub email: String,
    pub password: String,
    pub name: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct LoginUser {
    pub email: String,
    pub password: String,
}
