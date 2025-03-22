use crate::errors::ServiceError;
use argon2::{self, Config, Variant, Version};
use once_cell::sync::Lazy;
use rand::Rng;

use super::config::CONFIG;

pub fn hash_password(password: &str) -> Result<String, ServiceError> {
    let secret_key = CONFIG.secret_key.clone();
    let mut salt = [0u8; 16];
    rand::rng().fill(&mut salt);

    let config = Config {
        variant: Variant::Argon2id,
        version: Version::Version13,
        mem_cost: 19456,
        time_cost: 2,
        lanes: 4,
        secret: secret_key.as_bytes(),
        ad: &[],
        hash_length: 32,
    };

    argon2::hash_encoded(password.as_bytes(), &salt, &config).map_err(|err| {
        dbg!(err);
        ServiceError::InternalServerError
    })
}

pub fn verify(hash: &str, password: &str) -> Result<bool, ServiceError> {
    let secret_key = CONFIG.secret_key.clone();
    argon2::verify_encoded_ext(hash, password.as_bytes(), secret_key.as_bytes(), &[]).map_err(
        |err| {
            dbg!(err);
            ServiceError::Unauthorized
        },
    )
}
