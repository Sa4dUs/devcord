use axum::{
    RequestExt,
    body::Body,
    http::{Request, StatusCode},
    middleware::Next,
    response::Response,
};
use axum_extra::{
    TypedHeader,
    headers::{Authorization, authorization::Bearer},
};
use jsonwebtoken::{DecodingKey, Validation, decode};
use uuid::Uuid;

use crate::jwt::Claims;

#[derive(Debug, Clone)]
pub struct AuthUser {
    pub user_id: Uuid,
}

pub async fn require_auth(mut req: Request<Body>, next: Next) -> Result<Response, StatusCode> {
    let TypedHeader(Authorization(bearer)) = req
        .extract_parts::<TypedHeader<Authorization<Bearer>>>()
        .await
        .map_err(|_| StatusCode::UNAUTHORIZED)?;

    let secret = std::env::var("JWT_SECRET").map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let token_data = decode::<Claims>(
        bearer.token(),
        &DecodingKey::from_secret(secret.as_ref()),
        &Validation::default(),
    )
    .map_err(|_| StatusCode::UNAUTHORIZED)?;

    let user_id =
        Uuid::parse_str(&token_data.claims.user_id).map_err(|_| StatusCode::UNAUTHORIZED)?;

    req.extensions_mut().insert(AuthUser { user_id });

    Ok(next.run(req).await)
}
