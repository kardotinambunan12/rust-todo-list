use serde::{Serialize, Deserialize};

#[derive(Deserialize)]
pub struct LoginInput {
    pub email: String,
    pub password: String,
}

#[derive(Serialize)]
pub struct LoginResponse {
    pub token: String,
    pub refresh_token: String,
    pub refresh_expired_at: usize,
    pub expired_at:usize,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RegisterRequest{
    pub email:String,
    pub password: String,
    pub nama_lengkap:String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RegisterResponse {
    pub messages:String,
    pub status_code:i32,
}