use axum::{
    Router,
    http::{HeaderValue, Method},
    routing::any,
};
use dotenv::var;
use hyper::header;
use tower_http::cors::CorsLayer;
use tower_http::trace::TraceLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use crate::{
    config::Config,
    middleware::{
        auth::AuthLayer, parser::ParserLayer, rate_limit::RateLimitLayer, router::RouterLayer,
    },
    state::AppState,
};

pub mod config;
pub(crate) mod handler;
pub(crate) mod jwt;
pub(crate) mod middleware;
pub(crate) mod state;

pub fn app(config: Config) -> Router {
    let state = AppState::new(config);

    let origins: Vec<HeaderValue> = var("CORS_ORIGIN")
        .expect("CORS_ORIGIN env not set")
        .split(",")
        .map(|e| e.trim().parse::<HeaderValue>())
        .collect::<Result<_, _>>()
        .expect("Invalid CORS_ORIGIN format");

    let cors_layer = CorsLayer::new()
        .allow_origin(origins)
        .allow_credentials(true)
        .allow_methods([Method::GET, Method::POST])
        .allow_headers([
            header::CONTENT_TYPE,
            header::AUTHORIZATION,
            header::ACCESS_CONTROL_ALLOW_ORIGIN,
        ]);

    Router::new()
        .route("/{*path}", any(handler::handler))
        .layer(TraceLayer::new_for_http())
        .layer(cors_layer)
        .layer(RateLimitLayer)
        .layer(AuthLayer {
            state: state.clone(),
        })
        .layer(RouterLayer {
            state: state.clone(),
        })
        .layer(ParserLayer)
        .with_state(state)
}

pub async fn run() -> anyhow::Result<()> {
    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer())
        .init();

    let config = config::load()?;
    let app = app(config);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;
    axum::serve(listener, app).await?;

    Ok(())
}
