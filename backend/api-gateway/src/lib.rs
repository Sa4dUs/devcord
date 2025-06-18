use std::sync::Arc;

use axum::{Extension, Router, routing::any};
use tower::layer;
use tower_http::trace::TraceLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use crate::middleware::{
    auth::AuthLayer, parser::ParserLayer, rate_limit::RateLimitLayer, router::RouterLayer,
};

pub(crate) mod config;
pub(crate) mod handler;
pub(crate) mod middleware;

pub async fn run() -> anyhow::Result<()> {
    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer())
        .init();

    let config = Arc::new(config::load()?);
    let app = Router::new()
        .route("/{*path}", any(handler::handler))
        .layer(TraceLayer::new_for_http())
        .layer(RateLimitLayer)
        .layer(ParserLayer)
        .layer(Extension(config))
        .layer(RouterLayer)
        .layer(AuthLayer);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;
    axum::serve(listener, app).await?;

    Ok(())
}
