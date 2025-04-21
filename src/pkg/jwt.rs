
use jsonwebtoken::{decode, encode, DecodingKey, Header, Validation, errors::Result, EncodingKey};

use crate::model::jwt::Claims;

static SECRET:&[u8]=b"secret";

pub fn generate_token(email:&str)->Result<(String, usize)>{
    let claims = Claims::new(email);
    let exp = claims.exp;
    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(SECRET)
    )?;
   Ok((token, exp))
}

pub fn validate_token(token:&str)-> Result<Claims>{
    decode::<Claims>(
        token,
        &DecodingKey::from_secret(SECRET),
        &Validation::default(),
    )
        .map(|data|data.claims)
}