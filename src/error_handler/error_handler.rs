use axum::{response::{IntoResponse, Response}, http::StatusCode};

use axum::body::{Body};


#[derive(Debug)]
pub enum ApiError {
    NotFound,
    InternalServerError,
    BadRequest,
}

impl ApiError {
    fn message(&self) -> &str {
        match self {
            ApiError::NotFound => "Resource not found",
            ApiError::InternalServerError => "Internal server error",
            ApiError::BadRequest => "Bad request",
        }
    }

    fn status_code(&self) -> StatusCode {
        match self {
            ApiError::NotFound => StatusCode::NOT_FOUND,
            ApiError::InternalServerError => StatusCode::INTERNAL_SERVER_ERROR,
            ApiError::BadRequest => StatusCode::BAD_REQUEST,
        }
    }
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let message = self.message();
        let status = self.status_code();

        let body = Body::from(format!("{{\"error\":\"{}\"}}", message));

        Response::builder()
            .status(status)
            .header("Content-Type", "application/json")
            .body(body)
            .unwrap()
    }
}
