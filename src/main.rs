mod routes;
mod repository;
mod model;
mod pkg;
mod error_handler;
mod service;
mod controller;
mod middleware;
mod util;


use std::net::SocketAddr;
use axum::serve;
use tokio::net::TcpListener;
use crate::routes::router;
use crate::util::util::print_banner;
#[tokio::main]
async fn main() {
    let app = router::create_router();
    
    /* tracing::info!("🚀 Server running at {}", addr); */
    let addr= SocketAddr::from(([127, 0, 0, 1], 3000));
    print_banner(&addr);

    let listener = TcpListener::bind(addr).await.unwrap();

    serve(listener, app).await.unwrap();
}
