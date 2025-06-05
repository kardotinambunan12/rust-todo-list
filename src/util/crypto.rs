use crate::error_handler::error_handler::ApiError;
use base64::{decode, encode};
use rand::rngs::OsRng;
use rsa::{Oaep, RsaPrivateKey, RsaPublicKey};
use sha2::Sha256;

pub fn generate_rsa_keypair() -> Result<(RsaPrivateKey, RsaPublicKey), Box<dyn std::error::Error>> {
    let mut rng = OsRng;
    let bits = 2048;
    let private_key = RsaPrivateKey::new(&mut rng, bits)?;
    let public_key = RsaPublicKey::from(&private_key);
    Ok((private_key, public_key))
}
pub fn encrypt_rsa(public_key: &RsaPublicKey, data: &str) -> Result<String, ApiError> {
    let mut rng = rand::thread_rng();
    let encrypted_data = public_key
        .encrypt(&mut rng, Oaep::new::<Sha256>(), data.as_bytes())
        .map_err(|_| ApiError::InternalServerError)?;

    Ok(encode(&encrypted_data))
}
pub fn decrypt_rsa(private_key: &RsaPrivateKey, encrypted_base64: &str, ) -> Result<String, ApiError> {
    let encrypted_bytes = decode(encrypted_base64).map_err(|_| ApiError::InternalServerError)?;

    let decrypted_data = private_key
        .decrypt(Oaep::new::<Sha256>(), &encrypted_bytes)
        .map_err(|_| ApiError::InternalServerError)?;

    let decrypted_str =
        String::from_utf8(decrypted_data).map_err(|_| ApiError::InternalServerError)?;

    Ok(decrypted_str)
}
