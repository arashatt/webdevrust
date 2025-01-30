use axum::{Router, routing::get, Server};
use std::net::SocketAddr;
use tokio::sync::oneshot;
use tokio::signal;

#[tokio::main]
async fn main() {
    // Create a oneshot channel for shutdown signal
    let (shutdown_tx, shutdown_rx) = oneshot::channel();

    // Build the Axum router
    let app = Router::new().route("/", get(|| async { "Hello, world!" }));

    // Define the server address
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    // Spawn the server in a Tokio task
    let server = Server::bind(&addr)
        .serve(app.into_make_service())
        .with_graceful_shutdown(async {
            // Wait for the shutdown signal
            let _ = shutdown_rx.await;
            println!("Shutdown signal received, shutting down server...");
        });

    // Spawn the server task
    let server_handle = tokio::spawn(server);

    // Simulate some work before shutting down
    tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;

    // Send the shutdown signal
    let _ = shutdown_tx.send(());

    // Wait for the server to shut down
    let _ = server_handle.await;

    println!("Server has shut down.");
}
