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
use serde::{Deserialize, Serialize};
use sqlx::{mysql::MySqlPoolOptions, MySqlPool};
use std::{env, future::IntoFuture, sync::Arc};
use testcontainers_modules::mysql;
use testcontainers_modules::testcontainers::runners::AsyncRunner;
use tokio::sync::oneshot::{self};
#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    dotenv().ok();
    tracing_subscriber::fmt::init();
    let mysql_instance = mysql::Mysql::default().start().await.unwrap();
    //    let database_url = format!(
    //    "mysql://{}:{}/test",
    //    mysql_instance.get_host().unwrap(),
    //    mysql_instance.get_host_port_ipv4(3306).unwrap()
    // );
    print!(
        "{}   {}",
        mysql_instance.get_host().await.unwrap(),
        mysql_instance.get_host_port_ipv4(3306).await.unwrap()
    );
    let database_url = env::var("DATABASE_URL").expect("Didn't find mysql url");
    let pool = MySqlPoolOptions::new().connect(&database_url).await?;
    let app = Router::new()
        .route("/", get(root))
        .route("/get/{username}", get(username))
        .route("/test", get(|| async { "test" }))
        .route("/register", post(register))
        .route("/login", post(login))
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
    let response = register_user(new_user, &pool).await;
    dbg!(response);
    response
}
#[derive(Serialize, Deserialize)]
struct LoginForm {
    username: String,
    password: String,
}
use auth::register::login_user;
async fn login(
    State(pool): State<MySqlPool>,
    Json(new_user): Json<LoginForm>,
) -> impl IntoResponse {
    match login_user(
        new_user.username.as_str(),
        &new_user.password.as_str(),
        &pool,
    )
    .await
    {
        Ok(_) => (StatusCode::OK, "Correct".to_owned()),
        Err(err) => (StatusCode::FORBIDDEN, format!("{:#?}", err)),
    }
}
