use axum::{Json, http::StatusCode};
use axum::response::IntoResponse;
use tracing::info;
use crate::error_handler::error_handler::ApiError;
use crate::model::user::{LoginInput, LoginResponse};
use crate::pkg::jwt::generate_token;
use crate::util::util::hash_password;
use crate::service::auth::login_service::login_service;

pub async fn login(Json(payload): Json<LoginInput>) -> Result<Json<LoginResponse>, ApiError> {
    tracing::info!("Login attempt: {}", payload.email);

    let response  = login_service(payload).await?;
    Ok(Json(response))
}