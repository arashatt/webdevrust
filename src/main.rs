mod data;
mod auth;

use axum::{Router, routing::get, response::IntoResponse, extract::State};
use sqlx::{MySqlPool, mysql::MySqlPoolOptions};
use dotenvy::dotenv;
use std::env;


#[tokio::main]
async fn main() -> Result<(), sqlx::Error>{
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("Didn't find mysql url");
    let pool = MySqlPoolOptions::new().connect(&database_url).await?;
    let app = Router::new().route("/", get(root)).with_state(pool);
    let listen = tokio::net::TcpListener::bind("127.0.0.1:8000").await.unwrap();
    axum::serve(listen, app).await.unwrap();
    Ok(())
}

use data::person::foo;
async fn root(State(pool): State<MySqlPool>) -> impl IntoResponse{
    let a = foo(pool).await;
    format!("{:#?}", a)
}
