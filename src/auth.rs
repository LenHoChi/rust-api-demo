use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use std::env;
use uuid::Uuid;

// use crate::errors::AppError;
use crate::errors::errors::AppError;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,   // user id
    pub exp: usize,    // expiry timestamp
}

pub fn create_token(user_id: &Uuid) -> Result<String, AppError> {
    let secret = env::var("JWT_SECRET").unwrap_or_else(|_| "secret".to_string());
    let expires_in: i64 = env::var("JWT_EXPIRES_IN")
        .unwrap_or_else(|_| "86400".to_string())
        .parse()
        .unwrap_or(86400);

    let exp = (Utc::now() + Duration::seconds(expires_in)).timestamp() as usize;
    let claims = Claims { sub: user_id.to_string(), exp };

    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_bytes()),
    )
    .map_err(|_| AppError::InternalError)
}

pub fn verify_token(token: &str) -> Result<Claims, AppError> {
    let secret = env::var("JWT_SECRET").unwrap_or_else(|_| "secret".to_string());

    decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret.as_bytes()),
        &Validation::default(),
    )
    .map(|data| data.claims)
    .map_err(|e| match e.kind() {
        jsonwebtoken::errors::ErrorKind::ExpiredSignature => {
            AppError::Unauthorized("Token expired".to_string())
        }
        _ => AppError::Unauthorized("Invalid token".to_string()),
    })
}
