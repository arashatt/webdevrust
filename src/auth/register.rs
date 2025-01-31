use crate::data::person::User;
use argon2::password_hash::Error as Argon2Error;

use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use sqlx::Error as SqlxError;
use thiserror::Error;

use axum::http::StatusCode;
use sqlx::MySqlPool;
pub async fn register_user(new_user: User, pool: &MySqlPool) -> Result<StatusCode, StatusCode> {
    if (matches!(
        User::get_user_by_username(new_user.username.as_ref(), pool).await,
        Err(sqlx::Error::RowNotFound)
    ) && matches!(
        User::get_user_by_email(new_user.email.as_ref(), pool).await,
        Err(sqlx::Error::RowNotFound)
    )) {
        let salt = SaltString::generate(&mut OsRng);
        // Argon2 with default params (Argon2id v19)
        let argon2 = Argon2::default();

        // Hash password to PHC string ($argon2id$v=19$...)
        let password_hash = argon2
            .hash_password(new_user.password.as_bytes(), &salt)
            .unwrap()
            .to_string();
        let query = sqlx::query!(
            "INSERT INTO user (username, password, email) VALUES (?, ?, ?)",
            new_user.username,
            password_hash,
            new_user.email
        );
        query.execute(pool).await;
        Ok(StatusCode::CREATED)
    } else {
        Err(StatusCode::CONFLICT)
    }
}

#[derive(Debug, Error)]
pub enum LoginError {
    #[error("Database error: {0}")]
    Database(#[from] SqlxError),
    #[error("Invalid password hash: {0}")]
    Hash(String), // Store error details
    #[error("Invalid password")]
    Verification(String),
}

pub async fn login_user(
    login_user: &str,
    login_password: &str,
    pool: &MySqlPool,
) -> Result<(), LoginError> {
    let db_user = User::get_user_by_username(login_user, pool).await?;
    let parsed_hash =
        PasswordHash::new(&db_user.password).map_err(|e| LoginError::Hash(e.to_string()))?;

    Argon2::default()
        .verify_password(login_password.as_bytes(), &parsed_hash)
        .map_err(|e| LoginError::Verification(e.to_string()))
}
