use chrono::{Duration, Utc};
use jsonwebtoken::{EncodingKey, Header, encode};
use serde::{Deserialize, Serialize};
use std::env;

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct Claims {
    // TODO(Sa4dUs): Add more claim fields if needed
    exp: u64,
    // Actual payload
    user_id: String,
}

pub fn generate_jwt(user_id: String) -> Result<String, jsonwebtoken::errors::Error> {
    let expiration = Utc::now()
        .checked_add_signed(Duration::hours(24))
        .expect("timestamp")
        .timestamp() as u64;

    let claims = Claims {
        exp: expiration,
        user_id,
    };

    let secret = env::var("JWT_SECRET").expect("JWT_SECRET not defined in .env");

    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_bytes()),
    )
}
