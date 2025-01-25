use sqlx::{FromRow, MySqlPool};
use chrono::{DateTime, Local};

#[derive(FromRow, Debug)]
pub struct User{
    pub id: i32,
    pub username: String,
    pub email: String,
    pub password: String, 
    pub created_at: DateTime<Local>, // https://docs.rs/sqlx/latest/sqlx/mysql/types/index.html
}

pub async fn foo(pool:MySqlPool ) -> Vec<User> {
sqlx::query_as::<_, User>("SELECT * FROM user")
  .fetch_all(&pool).await.unwrap()

}

struct TimeSpent(u64);

