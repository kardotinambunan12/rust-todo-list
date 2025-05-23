use crate::error_handler::error_handler::ApiError;
use crate::model::response::user_response::User;
use crate::pkg::db::db_connection;
use mysql::params;
use mysql::prelude::Queryable;
use crate::model::user::{RegisterRequest, RegisterResponse};

pub async fn find_email_user(email: &str) -> Result<Option<User>, ApiError> {
    let mut conn = db_connection();
    let result = conn
        .exec_first::<(i32, String, String), _, _>(
            "SELECT id, email, password FROM users WHERE email = :email",
            params! { "email" => email },
        )
        .map_err(|_| ApiError::InternalServerError)?;

    Ok(result.map(|(id, email, password)| User {
        id,
        email,
        password,
    }))
}

pub async fn create_user_register(register_request: RegisterRequest) -> Result<Option<RegisterResponse>, ApiError> {

    let mut conn = db_connection();
    println!("connection {:?}", conn);

    conn.exec_drop(
        "INSERT INTO `todo-list`.users (email, password, nama_lengkap) VALUES (:email, :password, :nama_lengkap)",
        params! {
            "email" => &register_request.email,
            "password" => &register_request.password,
            "nama_lengkap" => &register_request.nama_lengkap,
        },


    ).map_err(|e| {
        println!("DB insert error: {:?}", e);
        ApiError::InternalServerError
    })?;

    let id = conn.last_insert_id() as i32;
    if id == 0 {
        return Err(ApiError::NotFound);
    }


    let response = RegisterResponse {
        messages: "success".to_string(),
        status_code: 200,
    };

    Ok(Some(response))
}
