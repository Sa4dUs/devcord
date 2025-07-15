use std::{
    pin::Pin,
    sync::Arc,
    task::{Context, Poll},
    time::Instant,
};

use axum::{
    extract::Request,
    response::{IntoResponse, Response},
};
use dashmap::DashMap;
use hyper::StatusCode;
use tower::{Layer, Service};

use crate::{
    circuit_breaker::{self, CircuitBreaker},
    config::Instance,
    error::ErrorResponse,
    load_balancer::LoadBalancer,
};

#[derive(Clone)]
pub struct LoadBalancerLayer {
    circuit_breaker: Arc<DashMap<String, CircuitBreaker>>,
}

impl LoadBalancerLayer {
    pub fn with_circuit_breaker() -> Self {
        Self {
            circuit_breaker: Arc::new(DashMap::new()),
        }
    }
}

impl<S> Layer<S> for LoadBalancerLayer {
    type Service = LoadBalancerMiddleware<S>;

    fn layer(&self, inner: S) -> Self::Service {
        LoadBalancerMiddleware::new(inner, self.circuit_breaker.clone())
    }
}

#[derive(Clone)]
pub struct LoadBalancerMiddleware<S> {
    inner: S,
    circuit_breaker: Arc<DashMap<String, CircuitBreaker>>,
}

impl<S> LoadBalancerMiddleware<S> {
    pub fn new(inner: S, circuit_breaker: Arc<DashMap<String, CircuitBreaker>>) -> Self {
        Self {
            inner,
            circuit_breaker,
        }
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
        let circuit_breaker = self.circuit_breaker.clone();

        Box::pin(async move {
            let service = match req.extensions().get::<crate::config::Service>() {
                Some(svc) => svc,
                None => {
                    return Ok(StatusCode::INTERNAL_SERVER_ERROR
                        .with_debug("Could not get `Service` extension at load balancer middleware")
                        .into_response());
                }
            };

            let lb = LoadBalancer::new(service.strategy.clone());
            let healthy_instances: Vec<_> = service
                .instances
                .iter()
                .filter(|Instance(uri)| match circuit_breaker.get(uri) {
                    Some(cb) => match cb.state {
                        circuit_breaker::CircuitState::Closed => true,
                        circuit_breaker::CircuitState::HalfOpen => true,
                        circuit_breaker::CircuitState::Open { retry_after } => {
                            Instant::now() >= retry_after
                        }
                    },
                    None => true,
                })
                .cloned()
                .collect();

            let instance = match lb.select_instance(&healthy_instances[..]) {
                Some(val) => val,
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
