use serde::Deserialize;

#[derive(Debug,Deserialize)]
pub struct User {
    pub id: i32,
    pub email:String,
    pub password:String,
}