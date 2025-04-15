
use mysql::params;
use mysql::prelude::Queryable;
use tracing::log::__private_api::log;
use crate::model::entity::{User, UserInput};
use crate::pkg::db::db_connection;
use crate::error_handler::error_handler::ApiError;

pub async fn get_all_user() -> Result<Vec<User>, ApiError> {
    let mut conn = db_connection();

    let result = conn
        .query_map("SELECT * FROM users", |(id, name)| User { id, name });

    match result {
        Ok(users)=>Ok(users),
        Err(_)=> Err(ApiError::InternalServerError),
    }

}


pub fn get_user_by_id(user_id: i32) -> Result<Option<User>, ApiError> {
    let mut conn = db_connection();

    let result = conn.exec_first::<(i32, String), _, _>(
        "SELECT id, name FROM users WHERE id = :id",
        params! { "id" => user_id },
    ).map_err(|_| ApiError::InternalServerError)?;

    // Convert tuple into User struct
    Ok(result.map(|(id, name)| User { id, name }))
}



pub async fn create_user(user: UserInput) -> Result<User, ApiError> {
    let mut conn = db_connection();
println!("connection {:?}", conn);
println!("log name {}", user.name);
    conn.exec_drop(
        "INSERT INTO `todo-list`.users (name) VALUES (:name)",
        params! { "name" => &user.name },
    ).map_err(|e| {
        eprintln!("DB Error: {}", e);
        ApiError::InternalServerError
    })?;


    let id = conn.last_insert_id() as i32;
    if id ==0 {
        return Err(ApiError::NotFound)
    }

    Ok(User { id, name: user.name })
}


