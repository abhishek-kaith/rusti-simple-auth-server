use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};

use crate::{AppError, Result};

pub fn hash_password(password: &str) -> Result<String> {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    let password_hash = argon2
        .hash_password(password.as_bytes(), &salt)
        .map_err(|_e| {
            tracing::error!("Failed to hash password");
            AppError::InternalServerError
        })?;
    Ok(password_hash.to_string())
}

pub fn verify_password(password: &str, password_hash: &str) -> Result<()> {
    let argon2 = Argon2::default();
    let pass_hash = PasswordHash::new(password_hash).map_err(|_e| {
        tracing::error!("Failed to parse password hash");
        AppError::InternalServerError
    })?;
    argon2.verify_password(password.as_bytes(), &pass_hash).map_err(|_e| {
        AppError::InvalidCredentials
    })
}
