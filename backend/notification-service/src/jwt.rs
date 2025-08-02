use std::sync::LazyLock;

use axum::{
    Json, RequestPartsExt,
    extract::FromRequestParts,
    http::{StatusCode, header, request::Parts},
    response::{IntoResponse, Response},
};
use axum_extra::{
    TypedHeader,
    headers::{Authorization, authorization::Bearer},
};
use jsonwebtoken::{DecodingKey, Validation, decode};
use serde::{Deserialize, Serialize};
use serde_json::json;

static KEYS: LazyLock<Keys> = LazyLock::new(|| {
    let secret = std::env::var("JWT_SECRET").expect("JWT_SECRET must be set");
    Keys::new(secret.as_bytes())
});

struct Keys {
    decoding: DecodingKey,
}

impl Keys {
    fn new(secret: &[u8]) -> Self {
        Self {
            decoding: DecodingKey::from_secret(secret),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct Claims {
    exp: u64,
    pub user_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct Authenticated {
    pub(crate) claims: Claims,
    pub(crate) jwt: String,
}

impl<S> FromRequestParts<S> for Authenticated
where
    S: Send + Sync,
{
    type Rejection = AuthError;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        if let Ok(TypedHeader(bearer)) = parts.extract::<TypedHeader<Authorization<Bearer>>>().await
        {
            let jwt = bearer.token().to_string();
            let claims = decode_token(&jwt)?;
            return Ok(Authenticated { claims, jwt });
        }

        // FIXME(Sa4dUs): Collapse these `if let` statements once `Dockerfile` runs rust 2024 edition
        if let Some(protocol_header) = parts.headers.get(header::SEC_WEBSOCKET_PROTOCOL) {
            if let Ok(protocols) = protocol_header.to_str() {
                if let Some(jwt) = protocols.split(',').map(|s| s.trim()).next() {
                    let claims = decode_token(jwt)?;
                    return Ok(Authenticated {
                        claims,
                        jwt: jwt.to_string(),
                    });
                }
            }
        }

        Err(AuthError::InvalidToken)
    }
}

fn decode_token(token: &str) -> Result<Claims, AuthError> {
    decode::<Claims>(token, &KEYS.decoding, &Validation::default())
        .map(|data| data.claims)
        .map_err(|_| AuthError::InvalidToken)
}

//This should be a common crate for all services, dead code is allowed to preserve the common structure
#[allow(dead_code)]
#[derive(Debug)]
pub enum AuthError {
    WrongCredentials,
    MissingCredentials,
    TokenCreation,
    InvalidToken,
}

impl IntoResponse for AuthError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            AuthError::WrongCredentials => (StatusCode::UNAUTHORIZED, "Wrong credentials"),
            AuthError::MissingCredentials => (StatusCode::BAD_REQUEST, "Missing credentials"),
            AuthError::TokenCreation => (StatusCode::INTERNAL_SERVER_ERROR, "Token creation error"),
            AuthError::InvalidToken => (StatusCode::BAD_REQUEST, "Invalid token"),
        };
        let body = Json(json!({
            "error": error_message,
        }));
        (status, body).into_response()
    }
}

#[derive(Debug, Serialize)]
pub struct AuthBody {
    access_token: String,
    token_type: String,
}
