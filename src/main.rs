mod routes;
mod repository;
mod model;
mod pkg;
mod error_handler;
mod service;
mod controller;

use std::net::SocketAddr;
use axum::serve;
use tokio::net::TcpListener;
use crate::routes::router;

#[tokio::main]
async fn main() {
    let app = router::create_router();

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    // tracing::info!("🚀 Server running at {}", addr);
    println!("listening on http:// {}", addr);

    let listener = TcpListener::bind(addr).await.unwrap();

    serve(listener, app).await.unwrap();
}
