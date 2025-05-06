use crate::error_handler::error_handler::ApiError;
use crate::model::entity::{User, UserInput};
use crate::service::todo_service;
use axum::{extract::Path, http::StatusCode, response::IntoResponse, Json};

pub async fn get_user_by_id_controller(Path(user_id): Path<i32>) -> Result<Json<User>, ApiError> {
    let user = todo_service::get_user_by_id_service(user_id)?;
    Ok(Json(user))
}

pub async fn get_all_users_controller() -> Result<Json<Vec<User>>, ApiError> {
    todo_service::get_all_users().await
}

pub async fn create_user_controller(Json(payload): Json<UserInput>) -> impl IntoResponse {
    match todo_service::create_user_service(payload).await {
        Ok(response) => (StatusCode::CREATED, Json(response)).into_response(),
        Err(err) => err.into_response(),
    }
}
