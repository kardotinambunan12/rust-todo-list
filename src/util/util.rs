
use sha1::{Digest,Sha1};

pub fn hash_password(password: &str) ->String{
    let mut hasher = Sha1::new();
    hasher.update(password.as_bytes());

    let result = hasher.finalize();
    hex::encode(result)
}