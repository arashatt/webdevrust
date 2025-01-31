use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, MySqlPool};

#[derive(FromRow, Debug, Serialize, Deserialize)]
pub struct User {
    #[sqlx(Default)]
    pub id: Option<i32>,
    pub username: String,
    pub email: String,
    pub password: String,
    pub created_at: Option<DateTime<Local>>, // https://docs.rs/sqlx/latest/sqlx/mysql/types/index.html
}
impl User {
    pub async fn foo(pool: &MySqlPool) -> Vec<User> {
        sqlx::query_as::<_, User>("SELECT * FROM user")
            .fetch_all(pool)
            .await
            .unwrap()
    }

    pub async fn get_user_by_username(
        username: &str,
        pool: &MySqlPool,
    ) -> Result<User, sqlx::Error> {
        sqlx::query_as::<_, User>("SELECT * FROM user where username=?")
            .bind(username)
            .fetch_one(pool)
            .await
    }
    pub async fn get_user_by_email(email: &str, pool: &MySqlPool) -> Result<User, sqlx::Error> {
        sqlx::query_as::<_, User>("SELECT * FROM user where email=?")
            .bind(email)
            .fetch_one(pool)
            .await
    }
}

// TODO: use time spent by request. middleware section.
