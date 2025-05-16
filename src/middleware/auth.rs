use crate::model::jwt::Claims;
use crate::pkg::jwt::validate_token;
use axum::{
    extract::FromRequestParts,
    http::{request::Parts, StatusCode},
};

pub struct Auth(pub Claims);

impl<S> FromRequestParts<S> for Auth
where
    S: Send + Sync,
{
    type Rejection = (StatusCode, String);
    async fn from_request_parts(parts: &mut Parts, _: &S) -> Result<Self, Self::Rejection> {
        let auth_header = parts
            .headers
            .get("Authorization")
            .and_then(|v| v.to_str().ok());

        if let Some(auth_header) = auth_header {
            if let Some(token) = auth_header.strip_prefix("Bearer") {
                match validate_token(token) {
                    Ok(claims) => return Ok(Auth(claims)),
                    Err(_) => return Err((StatusCode::UNAUTHORIZED, "Token invalid".into())),
                }
            }
        }
        Err((
            StatusCode::UNAUTHORIZED,
            "Authorization header missing".into(),
        ))
    }
}
