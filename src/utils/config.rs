use actix_identity::{config::LogoutBehaviour, IdentityMiddleware};
use actix_session::{
    config::{PersistentSession, SessionMiddlewareBuilder},
    storage::{CookieSessionStore, RedisSessionStore},
    SessionMiddleware,
};
use actix_web::cookie::{time::Duration as CookieDuration, Key};
use dotenv::dotenv;
use lazy_static::lazy_static;
use std::{env, time::Duration};

#[derive(Debug, Clone)]
pub enum SessionBackend {
    Redis(String),
    Cookies,
}

#[derive(Debug, Clone)]
pub struct Config {
    pub db_string: String,
    pub server_host: String,
    pub server_port: u16,
    pub session_backend: SessionBackend,
    pub redis_url: String,
    pub secret_key: String,
    pub cookie_ttl_secs: Duration,
}

lazy_static! {
    pub static ref CONFIG: Config = Config::from_env().expect("Failed to load configuration");
}

pub fn get_config() -> &'static Config {
    &CONFIG
}

#[derive(Debug, thiserror::Error)]
pub enum ConfigError {
    #[error("Missing environment variable: {0}")]
    MissingEnv(String),

    #[error("Invalid value for environment variable: {0}")]
    InvalidValue(String),

    #[error("Redis connection error: {0}")]
    RedisError(String),
}

impl Config {
    pub fn from_env() -> Result<Self, ConfigError> {
        dotenv().ok();

        let db_string = env::var("DATABASE_URL")
            .map_err(|_| ConfigError::MissingEnv("DATABASE_URL".to_string()))?;

        let server_port = env::var("SERVER_PORT")
            .map_err(|_| ConfigError::MissingEnv("SERVER_PORT".to_string()))?
            .parse::<u16>()
            .map_err(|_| ConfigError::InvalidValue("SERVER_PORT".to_string()))?;

        let server_host = env::var("SERVER_HOST")
            .map_err(|_| ConfigError::MissingEnv("SERVER_HOST".to_string()))?;

        let secret_key = env::var("SECRET_KEY")
            .map_err(|_| ConfigError::MissingEnv("SECRET_KEY".to_string()))?;

        let session_backend_str = env::var("SESSION_BACKEND")
            .map_err(|_| ConfigError::MissingEnv("SESSION_BACKEND".to_string()))?
            .to_lowercase();

        let session_backend = match session_backend_str.as_str() {
            "redis" => {
                let redis_url = env::var("REDIS_URL")
                    .map_err(|_| ConfigError::MissingEnv("REDIS_URL".to_string()))?;
                SessionBackend::Redis(redis_url)
            }
            "cookies" => SessionBackend::Cookies,
            _ => {
                return Err(ConfigError::InvalidValue(format!(
                    "SESSION_BACKEND: '{}' is not a valid value (must be 'redis' or 'cookies')",
                    session_backend_str
                )))
            }
        };

        let redis_url =
            env::var("REDIS_URL").map_err(|_| ConfigError::MissingEnv("REDIS_URL".to_string()))?;

        let cookie_ttl_secs = env::var("COOKIE_TTL_SECS")
            .map_err(|_| ConfigError::MissingEnv("COOKIE_TTL_SECS".to_string()))?
            .parse::<u64>()
            .map_err(|_| ConfigError::InvalidValue("COOKIE_TTL_SECS".to_string()))?;

        let cookie_ttl = Duration::from_secs(cookie_ttl_secs);

        Ok(Config {
            db_string,
            server_host,
            server_port,
            session_backend,
            redis_url,
            secret_key,
            cookie_ttl_secs: cookie_ttl,
        })
    }

    pub fn get_key(&self) -> Key {
        Key::from(self.secret_key.as_bytes())
    }

    pub fn cookie_ttl_as_cookie_duration(&self) -> CookieDuration {
        CookieDuration::seconds(self.cookie_ttl_secs.as_secs() as i64)
    }
}

pub fn identity_middleware(config: &Config) -> IdentityMiddleware {
    IdentityMiddleware::builder()
        .visit_deadline(Some(config.cookie_ttl_secs))
        .login_deadline(Some(config.cookie_ttl_secs * 3))
        .logout_behaviour(LogoutBehaviour::PurgeSession)
        .build()
}

fn configure_session_middleware<S>(
    builder: SessionMiddlewareBuilder<S>,
    config: &Config,
) -> SessionMiddlewareBuilder<S>
where
    S: actix_session::storage::SessionStore,
{
    builder.cookie_secure(false).session_lifecycle(
        PersistentSession::default().session_ttl(config.cookie_ttl_as_cookie_duration()),
    )
}

pub async fn redis_session_middleware(
    config: &Config,
    redis_url: &str,
) -> Result<SessionMiddleware<RedisSessionStore>, ConfigError> {
    let redis_store = RedisSessionStore::new(redis_url)
        .await
        .map_err(|e| ConfigError::RedisError(e.to_string()))?;

    let builder = SessionMiddleware::builder(redis_store, config.get_key());
    Ok(configure_session_middleware(builder, config).build())
}

pub fn cookie_session_middleware(config: &Config) -> SessionMiddleware<CookieSessionStore> {
    let cookie_store = CookieSessionStore::default();
    let builder = SessionMiddleware::builder(cookie_store, config.get_key());
    configure_session_middleware(builder, config).build()
}
