use axum::{
    Json, RequestExt,
    body::Body,
    http::{Request, StatusCode, header},
    middleware::Next,
    response::{IntoResponse, Response},
};
use axum_extra::{
    TypedHeader,
    headers::{Authorization, authorization::Bearer},
};
use jsonwebtoken::{DecodingKey, Validation, decode};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::sync::LazyLock;

use crate::models::claims::Claims;

// FIXME(Sa4dUs): We should definetly extract JWT logic to a external crate

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct Authenticated {
    pub(crate) claims: Claims,
    pub(crate) jwt: String,
}

static KEYS: LazyLock<DecodingKey> = LazyLock::new(|| {
    let secret = std::env::var("JWT_SECRET").expect("JWT_SECRET must be set");
    DecodingKey::from_secret(secret.as_bytes())
});

pub async fn require_auth(mut req: Request<Body>, next: Next) -> Result<Response, AuthError> {
    let jwt = if let Ok(TypedHeader(Authorization(bearer))) = req
        .extract_parts::<TypedHeader<Authorization<Bearer>>>()
        .await
    {
        Some(bearer.token().to_string())
    } else if let Some(protocol_header) = req.headers().get(header::SEC_WEBSOCKET_PROTOCOL) {
        protocol_header
            .to_str()
            .ok()
            .and_then(|protocols| protocols.split(',').map(str::trim).next().map(String::from))
    } else {
        None
    };

    let jwt = jwt.ok_or(AuthError::InvalidToken)?;

    let token_data = decode::<Claims>(&jwt, &KEYS, &Validation::default())
        .map_err(|_| AuthError::InvalidToken)?;

    let authenticated = Authenticated {
        claims: token_data.claims,
        jwt,
    };

    req.extensions_mut().insert(authenticated);

    Ok(next.run(req).await)
}

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
