use std::{
    pin::Pin,
    task::{Context, Poll},
};

use axum::{
    RequestExt,
    extract::Request,
    response::{IntoResponse, Response},
};
use axum_extra::{
    TypedHeader,
    headers::{Authorization, authorization::Bearer},
};
use hyper::StatusCode;
use jsonwebtoken::{DecodingKey, Validation, decode};
use tower::{Layer, Service};

use crate::{error::ErrorResponse, jwt::Claims, middleware::parser::ParsedURI, state::AppState};

#[derive(Clone)]
pub(crate) struct AuthLayer {
    pub(crate) state: AppState,
}

impl<S> Layer<S> for AuthLayer {
    type Service = AuthMiddleware<S>;

    fn layer(&self, inner: S) -> Self::Service {
        AuthMiddleware {
            inner,
            state: self.state.clone(),
        }
    }
}

#[derive(Clone)]
pub struct AuthMiddleware<S> {
    inner: S,
    state: AppState,
}

impl<S> Service<Request> for AuthMiddleware<S>
where
    S: Service<Request, Response = Response> + Clone + Send + 'static,
    S::Future: Send + 'static,
{
    type Response = Response;
    type Error = S::Error;
    type Future =
        Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send + 'static>>;
    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx)
    }

    fn call(&mut self, mut req: Request) -> Self::Future {
        let mut inner = self.inner.clone();
        let AppState { config } = self.state.clone();

        Box::pin(async move {
            let ParsedURI { prefix, subpath } = match req.extensions().get::<ParsedURI>() {
                Some(uri) => uri,
                None => {
                    return Ok(StatusCode::INTERNAL_SERVER_ERROR
                        .with_debug("Could not get `ParsedURI` extension at auth middleware")
                        .into_response());
                }
            };

            let route = match config.get_route(prefix, subpath) {
                Some(r) => r,
                None => return Ok(StatusCode::NOT_FOUND
                    .with_debug(
                        "This is a bug! Unexpected route should be handled by router middleware.",
                    )
                    .into_response()),
            };

            if !route.protected {
                // If route isn't protected, we don't require the client to send
                // `Authorization: Bearer {TOKEN}` header, just continue
                let response = inner.call(req).await?;
                return Ok(response);
            }

            // Check `Authorization: Bearer {TOKEN}`
            let TypedHeader(Authorization(bearer)) = match req
                .extract_parts::<TypedHeader<Authorization<Bearer>>>()
                .await
            {
                Ok(bearer) => bearer,
                Err(_) => {
                    return Ok(StatusCode::UNAUTHORIZED
                        .with_debug("Authorization header with valid format must be providen for protected routes")
                        .into_response());
                }
            };

            let secret = std::env::var("JWT_SECRET").expect("env variable JWT_SECRET is not set");

            if decode::<Claims>(
                bearer.token(),
                &DecodingKey::from_secret(secret.as_ref()),
                &Validation::default(),
            )
            .is_err()
            {
                return Ok(StatusCode::UNAUTHORIZED
                    .with_debug("Invalid Authorization header. Could not decode into claim")
                    .into_response());
            };

            // Valid JWT token, pass
            let response = inner.call(req).await?;
            Ok(response)
        })
    }
}
