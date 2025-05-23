
use sha1::{Digest,Sha1};
use std::net::SocketAddr;

pub fn hash_password(password: &str) ->String{
    let mut hasher = Sha1::new();
    hasher.update(password.as_bytes());

    let result = hasher.finalize();
    hex::encode(result)
}

pub fn verify_password(password:&str, password_hash:&str) -> bool {
  hash_password(password) == password_hash

}

pub fn print_banner(addr: &SocketAddr) {
    println!();
    println!("#**************************************************#");
    println!("*                                                  *");
    println!("*        🚀  Rust Server is Running Smoothly!      *");
    println!("*                                                  *");
    println!("*    Listening on: http://{:^25}*", addr);
    println!("*                                                  *");
    println!("*           Press Ctrl+C to stop server            *");
    println!("*                                                  *");
    println!("#**************************************************#");
    println!();
}

