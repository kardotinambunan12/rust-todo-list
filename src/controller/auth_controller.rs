use axum::{Json, http::StatusCode, response::IntoResponse};
use crate::error_handler::error_handler::ApiError;
use crate::model::user::{LoginInput, LoginResponse, RegisterRequest};
use crate::util::util::hash_password;
use crate::service::auth::login_service::login_service;
use crate::service::auth::login_service::register_service;

use crate::util::crypto::{
    generate_rsa_keypair, encrypt_rsa, decrypt_rsa
};


pub async fn login(Json(mut payload): Json<LoginInput>) -> Result<Json<LoginResponse>, ApiError> {
    tracing::info!("Login attempt: {}", payload.email);

    let Ok((private_key, public_key)) = generate_rsa_keypair() else { todo!() };
    let encrypted= encrypt_rsa(&public_key, payload.email.as_str())?;
    let decrypted= decrypt_rsa(&private_key, &encrypted)?;
    println!("Encrypted: {}", encrypted);
    println!("Decrypted: {}", decrypted);

    payload.password = hash_password(&payload.password);

    let response  = login_service(payload).await?;
    Ok(Json(response))
}
pub async fn register_user_controller(Json(mut payload): Json<RegisterRequest>) -> impl IntoResponse {
    payload.password = hash_password(&payload.password);

    match register_service(payload).await {
        Ok(response) => (StatusCode::CREATED, Json(response)).into_response(),
        Err(err) => err.into_response(),
    }
}


