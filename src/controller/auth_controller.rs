use axum::{Json, http::StatusCode};
use tracing::info;
use crate::model::user::{LoginInput, LoginResponse};
use crate::pkg::jwt::generate_token;
use crate::util::util::hash_password;

pub async fn login(Json(payload): Json<LoginInput>) -> Result<Json<LoginResponse>, (StatusCode, String)> {
    println!("login");
    let password = "password";
    let hashed = hash_password(password);
    info!("hashed: {}", hashed);
        info!("hashed: {} {}", "LOG- TRC", "002");
    //ambil db dari  untuk data  untuk melakukan validasi data
    if payload.email == "admin@example.com" && payload.password == "password" {
        let (token, exp) = generate_token(&payload.email).map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "Token error".into()))?;
        Ok(Json(LoginResponse {
            token,
            expired_at:exp,
        }))
    } else {
        Err((StatusCode::UNAUTHORIZED, "Invalid credentials".into()))
    }
}