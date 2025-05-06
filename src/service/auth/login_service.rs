use crate::error_handler::error_handler::ApiError;

use crate::model::user::{LoginInput, LoginResponse};
use crate::pkg::jwt::generate_token;
use crate::repository::auth::login::find_email_user;
use crate::util::util::verify_password;

pub async fn login_service(login_input: LoginInput) -> Result<LoginResponse, ApiError> {
    let user = find_email_user(&login_input.email)
        .await
        .map_err(|_| ApiError::InternalServerError)?;

    let user = match user {
        Some(user) => user,
        None => return Err(ApiError::BadRequest),
    };
    if verify_password(&login_input.password, &user.password) {
        let (token, exp) =
            generate_token(&user.email).map_err(|_| ApiError::InternalServerError)?;

        Ok(LoginResponse {
            token,
            expired_at: exp,
        })
    } else {
        Err(ApiError::Unauthorized)
    }
}
