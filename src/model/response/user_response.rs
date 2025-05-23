use serde::Deserialize;

#[derive(Debug,Deserialize)]
#[allow(dead_code)]
pub struct User {
    pub id: i32,
    pub email:String,
    pub password:String,
}