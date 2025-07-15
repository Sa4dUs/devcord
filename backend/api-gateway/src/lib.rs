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
        auth::AuthLayer, load_balancer::LoadBalancerLayer, parser::ParserLayer,
        rate_limit::RateLimitLayer, router::RouterLayer,
    },
    state::AppState,
};

pub mod config;
pub(crate) mod error;
pub(crate) mod handler;
pub(crate) mod jwt;
pub(crate) mod load_balancer;
pub(crate) mod middleware;
pub(crate) mod state;

pub fn app(config: Config) -> Router {
    let state = AppState::new(config.clone());

    let origins: Vec<HeaderValue> = var("CORS_ORIGIN")
        .expect("CORS_ORIGIN env not set")
        .split(",")
        .map(|e| e.trim().parse::<HeaderValue>())
        .collect::<Result<_, _>>()
        .expect("Invalid CORS_ORIGIN format");

    let cors_layer = CorsLayer::new()
        .allow_origin(origins)
        .allow_credentials(true)
        .allow_methods([
            Method::GET,
            Method::HEAD,
            Method::POST,
            Method::PUT,
            Method::DELETE,
            Method::CONNECT,
            Method::OPTIONS,
            Method::TRACE,
            Method::PATCH,
        ])
        .allow_headers([
            header::CONTENT_TYPE,
            header::AUTHORIZATION,
            header::ACCESS_CONTROL_ALLOW_ORIGIN,
        ]);

    Router::new()
        .route("/api/{*path}", any(handler::http_handler))
        .route("/ws/{*path}", any(handler::ws_handler))
        .layer(TraceLayer::new_for_http())
        .layer(RateLimitLayer::from_config(config.rate_limit))
        .layer(LoadBalancerLayer)
        .layer(AuthLayer {
            state: state.clone(),
        })
        .layer(RouterLayer {
            state: state.clone(),
        })
        .layer(ParserLayer)
        .layer(cors_layer)
        .with_state(state)
}

pub async fn run() -> anyhow::Result<()> {
    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer())
        .init();

    let config = config::load()?;
    let app = app(config);

    let port: String = dotenv::var("PORT").unwrap_or("3000".to_owned());
    let addr = format!("0.0.0.0:{port}");
    tracing::info!("API Gateway listening on port {addr}");
    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}
