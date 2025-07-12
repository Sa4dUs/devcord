use std::{
    pin::Pin,
    task::{Context, Poll},
};

use axum::{
    extract::Request,
    response::{IntoResponse, Response},
};
use hyper::StatusCode;
use tower::{Layer, Service};

use crate::{error::ErrorResponse, middleware::parser::ParsedURI, state::AppState};

#[derive(Clone)]
pub(crate) struct RouterLayer {
    pub(crate) state: AppState,
}

impl<S> Layer<S> for RouterLayer {
    type Service = RouterMiddleware<S>;

    fn layer(&self, inner: S) -> Self::Service {
        RouterMiddleware {
            inner,
            state: self.state.clone(),
        }
    }
}

#[derive(Clone)]
pub struct RouterMiddleware<S> {
    inner: S,
    state: AppState,
}

impl<S> Service<Request> for RouterMiddleware<S>
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
                        .with_debug("Could not get `ParsedURI` extension at router middleware")
                        .into_response());
                }
            };

            let service = match config.services.get(prefix) {
                Some(svc) => svc,
                None => {
                    return Ok(StatusCode::NOT_FOUND
                        .with_debug("Could not get service. Service not found")
                        .into_response());
                }
            };

            let route = match service.routes.iter().find(|r| r.path == *subpath) {
                Some(r) => r,
                None => {
                    return Ok(StatusCode::NOT_FOUND
                        .with_debug("Could not get route. Route not found")
                        .into_response());
                }
            };

            match route
                .allow_methods
                .iter()
                .find(|m| m.eq_ignore_ascii_case(req.method().as_str()))
            {
                Some(_) => {}
                None => {
                    return Ok(StatusCode::METHOD_NOT_ALLOWED
                        .with_debug("Method not allowed")
                        .into_response());
                }
            };

            req.extensions_mut().insert(service.clone());

            let response = inner.call(req).await?;
            Ok(response)
        })
    }
}
