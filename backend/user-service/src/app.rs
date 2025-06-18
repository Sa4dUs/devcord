use std::{net::SocketAddr, sync::Arc};

use axum::{http::{header, HeaderValue, Method}, routing::post, serve, Router};
use dotenvy::dotenv;
use sqlx::postgres::PgPoolOptions;
use tokio::net::TcpListener;
use tower_http::{cors::CorsLayer, trace::TraceLayer};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use crate::update::update;

#[derive(Debug, Clone)]
pub struct AppState {
    pub db: sqlx::PgPool,
}

pub async fn run() -> anyhow::Result<()> {
    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer())
        .init();

    dotenv().ok();

    let cors_layer = CorsLayer::new()
        .allow_origin(&std::env::var("CORS_ORIGIN")?
            .split(",")
            .map(|e | e.parse::<HeaderValue>()? )
        )
        .allow_credentials(true)
        .allow_methods([Method::GET, Method::POST])
        .allow_headers([header::CONTENT_TYPE, header::AUTHORIZATION]);

    let trace_layer = TraceLayer::new_for_http();

    let db = PgPoolOptions::new()
        .connect(&std::env::var("DATABASE_URL")?)
        .await?;

    let state = Arc::new(AppState{db});

    let app = Router::new()
        .route("/update", post(update))
        .route("/request", || ())
        .route("/accept", || ())
        .layer(cors_layer)
        .layer(trace_layer)
        .with_state(state);

    let addr = SocketAddr::from(&std::env::var("SOCKET_ADDR")?);
    let listener = TcpListener::bind(addr).await?;

    println!("Server runnnig at: {addr}");

    serve(listener, app.into_make_service()).await?;
    Ok(())
}