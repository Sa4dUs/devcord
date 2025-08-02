use axum::{extract::Request, response::Response};
use std::{
    pin::Pin,
    task::{Context, Poll},
};
use tower::{Layer, Service};

#[derive(Clone, Debug)]
pub(crate) struct ParsedURI {
    pub(crate) prefix: String,
    pub(crate) subpath: String,
}

#[derive(Clone)]
pub struct ParserLayer;

impl<S> Layer<S> for ParserLayer {
    type Service = ParserMiddleware<S>;

    fn layer(&self, inner: S) -> Self::Service {
        ParserMiddleware::new(inner)
    }
}

#[derive(Clone)]
pub struct ParserMiddleware<S> {
    inner: S,
}

impl<S> ParserMiddleware<S> {
    pub fn new(inner: S) -> Self {
        Self { inner }
    }
}

impl<S> Service<Request> for ParserMiddleware<S>
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
        let full_path = req.uri().path();
        let mut segments = full_path.trim_start_matches('/').splitn(3, '/');
        let _protocol = segments.next().unwrap_or("api");
        let prefix = segments.next().unwrap_or("").to_string();
        let subpath = segments
            .next()
            .map(|s| format!("/{s}"))
            .unwrap_or("/".to_string());

        req.extensions_mut().insert(ParsedURI { prefix, subpath });

        let mut inner = self.inner.clone();

        Box::pin(async move {
            let response = inner.call(req).await?;
            Ok(response)
        })
    }
}
