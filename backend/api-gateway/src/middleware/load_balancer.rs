use std::{
    pin::Pin,
    task::{Context, Poll},
};

use axum::{extract::Request, response::Response};
use tower::{Layer, Service};

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

    fn call(&mut self, req: Request) -> Self::Future {
        // TODO(Sa4dUs): Add LoadBalancerMiddleware logic
        let mut inner = self.inner.clone();

        Box::pin(async move {
            let response = inner.call(req).await?;
            Ok(response)
        })
    }
}
