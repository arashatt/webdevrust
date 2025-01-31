#[allow(dead_code)]
mod auth;
mod data;

use axum::{
    extract::{Path, State},
    handler,
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use dotenvy::dotenv;
use sqlx::{mysql::MySqlPoolOptions, MySqlPool};
use std::{env, future::IntoFuture, sync::Arc};
use tokio::sync::oneshot::{self};

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    dotenv().ok();
    tracing_subscriber::fmt::init();
    let database_url = env::var("DATABASE_URL").expect("Didn't find mysql url");
    let pool = MySqlPoolOptions::new().connect(&database_url).await?;
    let app = Router::new()
        .route("/", get(root))
        .route("/get/{username}", get(username))
        .route("/test", get(|| async { "test" }))
        .route("/register", post(register))
        .with_state(pool);
    let listen = tokio::net::TcpListener::bind("127.0.0.1:8000")
        .await
        .unwrap();
    let (tx, rx) = oneshot::channel();

    let server = axum::serve(listen, app).with_graceful_shutdown(async {
        match rx.await {
            Ok(message) => println!("{}", message),
            Err(_) => {}
        }
    });

    let handler = tokio::spawn(server.into_future());

    tokio::spawn(async {
        tokio::signal::ctrl_c().await.unwrap();
        tx.send("Finished").unwrap();
    });
    handler.await.unwrap().unwrap();
    Ok(())
}

use data::person::User;
async fn root(State(pool): State<MySqlPool>) -> impl IntoResponse {
    let a = User::foo(&pool).await;
    format!("{:#?}", a)
}

async fn username(
    State(pool): State<MySqlPool>,
    Path(user_name): Path<String>,
) -> impl IntoResponse {
    let a = User::get_user_by_username(user_name.as_ref(), &pool).await;
    match a {
        Ok(record) => (StatusCode::OK, format!("{:#?}", record)),
        Err(e) => (StatusCode::NOT_FOUND, format!("{:#?}", e)),
    }
}
use auth::register::register_user;
async fn register(State(pool): State<MySqlPool>, Json(new_user): Json<User>) -> impl IntoResponse {
let response =     register_user(new_user, &pool).await;
    dbg!(response);
response
}
