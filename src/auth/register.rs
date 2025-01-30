use crate::data::person::User;
use axum::http::StatusCode;
use chrono::Local;
use sqlx::{query, MySqlPool};

// let salt = SaltString::generate(&mut OsRng);
// // Argon2 with default params (Argon2id v19)
// let argon2 = Argon2::default();

pub async fn register_user(new_user: User, pool: &MySqlPool) -> Result<StatusCode, StatusCode> {
    match User::get_user_by_username(new_user.username.as_ref(), pool).await {
        Ok(_) => {
            dbg!("User exists");
            Err(StatusCode::CONFLICT)
        }
        Err(sqlx::Error::RowNotFound) => {
            let query = sqlx::query!(
                "INSERT INTO user (username, password, email, created_at) VALUES (?, ?, ?, ?)",
                new_user.username,
                new_user.password,
                new_user.email,
                new_user.created_at
            );
            query.execute(pool).await.unwrap();
            dbg!("User created");

            Ok(StatusCode::CREATED)
        }
        _ => {
            dbg!("Unknowd happened at user");
            Ok(StatusCode::CREATED)
        }
    }
}
