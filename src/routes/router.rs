use axum::{Router, routing::{get, post}};
use tower_http::trace::TraceLayer;
use tracing_subscriber::EnvFilter;


use crate::controller::todo_controller::{
    get_all_users_controller,
    get_user_by_id_controller,
    create_user_controller,

};
use crate::controller::auth_controller::{
    login,
};

pub fn create_router() -> Router {
    dotenv::dotenv().ok();
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    Router::new()
        .route("/todo-list/users", get(get_all_users_controller))
        .route("/todo-list/users/:id", get(get_user_by_id_controller))
        .route("/todo-list/users/create", post(create_user_controller))
        .route("/login",post(login))
        .layer(TraceLayer::new_for_http())
}
