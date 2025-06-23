use axum::{Router, routing::any};
use tower_http::trace::TraceLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use crate::{
    middleware::{
        auth::AuthLayer, parser::ParserLayer, rate_limit::RateLimitLayer, router::RouterLayer,
    },
    state::AppState,
};

pub(crate) mod config;
pub(crate) mod handler;
pub(crate) mod jwt;
pub(crate) mod middleware;
pub(crate) mod state;

pub async fn run() -> anyhow::Result<()> {
    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer())
        .init();

    let config = config::load()?;
    let state = AppState::new(config);

    let app = Router::new()
        .route("/{*path}", any(handler::handler))
        .layer(TraceLayer::new_for_http())
        .layer(RateLimitLayer)
        .layer(AuthLayer {
            state: state.clone(),
        })
        .layer(RouterLayer {
            state: state.clone(),
        })
        .layer(ParserLayer)
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;
    axum::serve(listener, app).await?;

    Ok(())
}
