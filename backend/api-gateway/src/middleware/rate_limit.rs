use std::{
    pin::Pin,
    sync::Arc,
    task::{Context, Poll},
    time::{Duration, Instant},
};

use axum::{
    extract::Request,
    http::HeaderValue,
    response::{IntoResponse, Response},
};
use dashmap::DashMap;
use hyper::StatusCode;
use tower::{Layer, Service};

use crate::config::RateLimitConfig;

#[derive(Clone)]
pub struct RateLimitLayer {
    pub(crate) storage: Arc<DashMap<String, ClientRecord>>,
    pub(crate) max_requests: u32,
    pub(crate) window: Duration,
}

impl RateLimitLayer {
    pub fn from_config(config: RateLimitConfig) -> RateLimitLayer {
        RateLimitLayer {
            storage: Arc::new(DashMap::new()),
            max_requests: config.max_requests,
            window: Duration::from_secs(config.window_seconds),
        }
    }
}

#[derive(Debug)]
pub struct ClientRecord {
    count: u32,
    window_start: Instant,
}

impl<S> Layer<S> for RateLimitLayer {
    type Service = RateLimitMiddleware<S>;

    fn layer(&self, inner: S) -> Self::Service {
        RateLimitMiddleware::new(
            inner,
            Arc::clone(&self.storage),
            self.max_requests,
            self.window,
        )
    }
}

#[derive(Clone)]
pub struct RateLimitMiddleware<S> {
    inner: S,
    storage: Arc<DashMap<String, ClientRecord>>,
    max_requests: u32,
    window: Duration,
}

impl<S> RateLimitMiddleware<S> {
    pub fn new(
        inner: S,
        storage: Arc<DashMap<String, ClientRecord>>,
        max_requests: u32,
        window: Duration,
    ) -> Self {
        Self {
            inner,
            storage,
            max_requests,
            window,
        }
    }
}

impl<S> Service<Request> for RateLimitMiddleware<S>
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
        let client_ip = req
            .headers()
            .get("x-forwarded-for")
            .and_then(|v| v.to_str().ok())
            .unwrap_or("unknown")
            .to_string();

        let storage = Arc::clone(&self.storage);
        let max_requests = self.max_requests;
        let window = self.window;

        let mut inner = self.inner.clone();

        Box::pin(async move {
            let now = Instant::now();
            let mut record = storage.entry(client_ip).or_insert(ClientRecord {
                count: 0,
                window_start: now,
            });

            if now.duration_since(record.window_start) > window {
                record.count = 1;
                record.window_start = now;
            } else {
                if record.count >= max_requests {
                    let retry_after = (window - now.duration_since(record.window_start)).as_secs();
                    let mut response = StatusCode::TOO_MANY_REQUESTS.into_response();
                    response.headers_mut().insert(
                        "Retry-After",
                        HeaderValue::from_str(&retry_after.to_string()).unwrap(),
                    );
                    return Ok(response);
                }
                record.count += 1
            }

            let response = inner.call(req).await?;
            Ok(response)
        })
    }
}
