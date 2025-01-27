mod data;
mod auth;

use axum::{extract::{Path, State}, response::IntoResponse, routing::get, Router, http::StatusCode};
use sqlx::{MySqlPool, mysql::MySqlPoolOptions};
use dotenvy::dotenv;
use std::env;


#[tokio::main]
async fn main() -> Result<(), sqlx::Error>{
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("Didn't find mysql url");
    let pool = MySqlPoolOptions::new().connect(&database_url).await?;
    let app = Router::new().route("/", get(root)).route("/get/{username}", get(username)).with_state(pool);
    let listen = tokio::net::TcpListener::bind("127.0.0.1:8000").await.unwrap();
    axum::serve(listen, app).await.unwrap();
    Ok(())
}
use data::person::User;
async fn root(State(pool): State<MySqlPool>) -> impl IntoResponse{
    let a = User::foo(pool).await;
    format!("{:#?}", a)

}

async fn username(State(pool):State<MySqlPool>, Path(user_name):Path<String>) -> impl IntoResponse {
let a = User::get_username(&user_name, pool).await;
    match a {
        Ok(record) =>
            (StatusCode::OK, format!("{:#?}", record)),
        Err(_) => (StatusCode::NOT_FOUND, "Not Found!".to_owned())
        }

}
