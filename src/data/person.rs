use sqlx::{FromRow, MySqlPool};

#[derive(FromRow, Debug)]
pub struct User{
    pub id: i32,
    pub name : String,
}

pub async fn foo(pool:MySqlPool ) -> Vec<User> {
sqlx::query_as::<_, User>("SELECT * FROM user")
  .fetch_all(&pool).await.unwrap()

}

struct TimeSpent(u64);

