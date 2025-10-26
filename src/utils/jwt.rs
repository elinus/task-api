use crate::error::{AppError, Result};
use chrono::{Duration, Utc};
use jsonwebtoken::{
    DecodingKey, EncodingKey, Header, Validation, decode, encode,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
    pub sub: String,   // Subject (user_id)
    pub email: String, // User email
    pub role: String,  // User role
    pub exp: i64,      // Expiration time (Unix timestamp)
    pub iat: i64,      // Issued at (Unix timestamp)
}

impl Claims {
    pub fn new(
        user_id: Uuid,
        email: String,
        role: String,
        expiration_hours: i64,
    ) -> Self {
        let now = Utc::now();
        let exp = now + Duration::hours(expiration_hours);

        Self {
            sub: user_id.to_string(),
            email,
            role,
            exp: exp.timestamp(),
            iat: now.timestamp(),
        }
    }

    pub fn user_id(&self) -> Result<Uuid> {
        Uuid::parse_str(&self.sub).map_err(|_| {
            AppError::InternalError("Invalid user ID in token".to_string())
        })
    }
}

pub fn create_token(claims: &Claims, secret: &str) -> Result<String> {
    encode(
        &Header::default(),
        claims,
        &EncodingKey::from_secret(secret.as_bytes()),
    )
    .map_err(|e| {
        AppError::InternalError(format!("Failed to create token: {}", e))
    })
}

pub fn verify_token(token: &str, secret: &str) -> Result<Claims> {
    decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret.as_bytes()),
        &Validation::default(),
    )
    .map(|data| data.claims)
    .map_err(|e| AppError::Unauthorized(format!("Invalid token: {}", e)))
}
