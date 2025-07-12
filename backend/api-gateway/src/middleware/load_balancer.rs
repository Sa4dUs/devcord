use std::{
    pin::Pin,
    task::{Context, Poll},
};

use axum::{
    extract::Request,
    response::{IntoResponse, Response},
};
use hyper::StatusCode;
use rand::seq::IndexedRandom;
use tower::{Layer, Service};

use crate::error::ErrorResponse;

#[derive(Clone)]
pub struct LoadBalancerLayer;

impl<S> Layer<S> for LoadBalancerLayer {
    type Service = LoadBalancerMiddleware<S>;

    fn layer(&self, inner: S) -> Self::Service {
        LoadBalancerMiddleware::new(inner)
    }
}

#[derive(Clone)]
pub struct LoadBalancerMiddleware<S> {
    inner: S,
}

impl<S> LoadBalancerMiddleware<S> {
    pub fn new(inner: S) -> Self {
        Self { inner }
    }
}

impl<S> Service<Request> for LoadBalancerMiddleware<S>
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

        Box::pin(async move {
            let service = match req.extensions().get::<crate::config::Service>() {
                Some(svc) => svc,
                None => {
                    return Ok(StatusCode::INTERNAL_SERVER_ERROR
                        .with_debug("Could not get `Service` extension at load balancer middleware")
                        .into_response());
                }
            };

            // FIXME(Sa4dUs): Add strategy pattern for different load balancing strats
            let instance = match service.instances.choose(&mut rand::rng()) {
                Some(uri) => uri.clone(),
                None => {
                    return Ok(StatusCode::INTERNAL_SERVER_ERROR
                        .with_debug("Could not get an instance following load balancer strategy")
                        .into_response());
                }
            };

            req.extensions_mut().insert(instance);

            let response = inner.call(req).await?;
            Ok(response)
        })
    }
}
