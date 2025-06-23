use std::{
    pin::Pin,
    task::{Context, Poll},
};

use axum::{
    extract::Request,
    response::{IntoResponse, Response},
};
use hyper::{StatusCode, header::AUTHORIZATION};
use jsonwebtoken::{DecodingKey, Validation, decode};
use tower::{Layer, Service};

use crate::{jwt::Claims, middleware::parser::ParsedURI, state::AppState};

// TODO(Sa4dUs): Maybe this can be a tuple struct?
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

    fn call(&mut self, req: Request) -> Self::Future {
        let mut inner = self.inner.clone();
        let AppState { config } = self.state.clone();

        Box::pin(async move {
            let ParsedURI { prefix, subpath } = match req.extensions().get::<ParsedURI>() {
                Some(uri) => uri,
                None => return Ok(StatusCode::INTERNAL_SERVER_ERROR.into_response()),
            };

            // TODO(Sa4dUs): Make this decent once `config.path` is a HashMap
            // TODO(Sa4dUs): Replace `unwrap` calls with better error handling
            // We can unwrap because the router middleware checks if it's a valid route
            // but it would be better to return some type of `bug!()` like message
            let route = config
                .services
                .get(prefix)
                .unwrap()
                .routes
                .iter()
                .find(|r| r.path == *subpath)
                .unwrap();

            if !route.protected {
                // If route isn't protected, we don't require the client to send
                // `Authorization: Bearer {TOKEN}` header, just continue
                let response = inner.call(req).await?;
                return Ok(response);
            }

            // Check `Authorization: Bearer {TOKEN}`
            let token = match req.headers().get(&AUTHORIZATION) {
                Some(val) => val.to_str().unwrap(), // TODO(Sa4dUs): Handle this error properly
                None => return Ok(StatusCode::UNAUTHORIZED.into_response()),
            };

            tracing::debug!("{token:?}");
            let secret = std::env::var("JWT_SECRET").expect("env variable JWT_SECRET is not set");

            if decode::<Claims>(
                token,
                &DecodingKey::from_secret(secret.as_ref()),
                &Validation::default(),
            )
            .is_err()
            {
                return Ok(StatusCode::UNAUTHORIZED.into_response());
            };

            // Valid JWT token, pass
            let response = inner.call(req).await?;
            Ok(response)
        })
    }
}
