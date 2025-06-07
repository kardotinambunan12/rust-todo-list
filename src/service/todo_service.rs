use crate::error_handler::error_handler::ApiError;
use crate::model::api_response::ApiResponse;
use crate::model::entity::{User, UserInput};
use crate::repository::todo_repository;
use axum::Json;
use base64::decode;
use crate::model::user::{UploadImageRequest, UploadImageResponse};

pub fn get_user_by_id_service(user_id: i32) -> Result<User, ApiError> {
    match todo_repository::get_user_by_id(user_id)? {
        Some(user) => Ok(user),
        None => Err(ApiError::NotFound),
    }
}

pub async fn get_all_users() -> Result<Json<Vec<User>>, ApiError> {
    let users = todo_repository::get_all_user().await?;
    Ok(Json(users))
}

pub async fn create_user_service(user_input: UserInput) -> Result<ApiResponse<User>, ApiError> {
    let user = todo_repository::create_user(user_input).await?;
    let response = ApiResponse {
        status: 201,
        message: "User created successfully".to_string(),
        data: user,
    };

    Ok(response)
}


pub async fn upload_base64_service(payload:UploadImageRequest)->Result<ApiResponse<UploadImageResponse>, ApiError> {
    let base64_data = payload.image.split(',').last().unwrap_or(&payload.image);

    let decode_bytes = decode(base64_data).map_err(|_| ApiError::BadRequest)?;


    let saved_file = todo_repository::save_file(&payload.filename, &decode_bytes)?;

    Ok(ApiResponse {
        status: 200,
        message: "Image uploaded successfully".to_string(),
        data: UploadImageResponse {
            filename: saved_file.clone(),
            image: format!("/assets/images/{}", saved_file),
        },
    })
}
