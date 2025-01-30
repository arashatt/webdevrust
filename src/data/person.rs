use chrono::{DateTime, Local};
use sqlx::{FromRow, MySqlPool};

#[derive(FromRow, Debug)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub email: String,
    pub password: String,
    pub created_at: DateTime<Local>, // https://docs.rs/sqlx/latest/sqlx/mysql/types/index.html
}

impl User {
    pub async fn foo(pool: MySqlPool) -> Vec<User> {
        sqlx::query_as::<_, User>("SELECT * FROM user")
            .fetch_all(&pool)
            .await
            .unwrap()
    }

    pub async fn get_user_by_username(
        username: &str,
        pool: MySqlPool,
    ) -> Result<User, sqlx::Error> {
        sqlx::query_as::<_, User>("SELECT * FROM user where username=?")
            .bind(username)
            .fetch_one(&pool)
            .await
    }
    pub async fn get_user_by_email(username: &str, pool: MySqlPool) -> Result<User, sqlx::Error> {
        sqlx::query_as::<_, User>("SELECT * FROM user where username=?")
            .bind(username)
            .fetch_one(&pool)
            .await
    }
}

// TODO: use time spent by request. middleware section.
