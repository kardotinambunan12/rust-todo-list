use mysql::params;
use mysql::prelude::Queryable;
use crate::error_handler::error_handler::ApiError;
use crate::model::response::user_response::User;
use crate::pkg::db::db_connection;

pub async fn find_email_user(email:&str) -> Result<Option<User>, ApiError>{
    let mut conn = db_connection();
 let result = conn.exec_first::<(i32, String, String), _, _>(
        "SELECT id, email, password FROM users WHERE email = :email",
        params! { "email" => email },
        ).map_err(|_| ApiError::InternalServerError)?;


    Ok(result.map(|(id, email, password)| User {
        id,
        email,
        password,
    }))
}