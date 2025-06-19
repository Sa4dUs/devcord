use std::{net::SocketAddr, sync::Arc};

use axum::{http::{header, HeaderValue, Method}, routing::post, serve, Router};
use dotenvy::dotenv;
use sqlx::{migrate::Migrator, postgres::PgPoolOptions};
use tokio::net::TcpListener;
use tower_http::{cors::CorsLayer, trace::TraceLayer};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use crate::{accept::accept, request::request, update::update};

static MIGRATOR: Migrator = sqlx::migrate!();

#[derive(Debug, Clone)]
pub struct AppState {
    pub db: sqlx::PgPool,
}

pub async fn run() -> anyhow::Result<()> {
    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer())
        .init();

    dotenv().ok();

    let origins: Vec<HeaderValue> = std::env::var("CORS_ORIGIN")
        .expect("CORS_ORIGIN env not set")
        .split(",")
        .map(|e| e.trim().parse::<HeaderValue>())
        .collect::<Result<_, _>>()?;

    let cors_layer = CorsLayer::new()
        .allow_origin(origins)
        .allow_credentials(true)
        .allow_methods([Method::GET, Method::POST])
        .allow_headers([header::CONTENT_TYPE, header::AUTHORIZATION]);

    let trace_layer = TraceLayer::new_for_http();

    let db = PgPoolOptions::new()
        .connect(&std::env::var("DATABASE_URL").expect("DATABASE_URL env not set"))
        .await?;

    MIGRATOR.run(&db).await?;

    let state = Arc::new(AppState{db});

    let app = Router::new()
        .route("/update", post(update))
        .route("/request", post(request))
        .route("/accept", post(accept))
        .layer(cors_layer)
        .layer(trace_layer)
        .with_state(state);

    let addr: SocketAddr = std::env::var("SOCKET_ADDR")
        .expect("SOCKET_ADDR env not set")
        .parse()?;
    let listener = TcpListener::bind(addr).await?;

    println!("Server runnnig at: {addr}");

    serve(listener, app.into_make_service()).await?;
    Ok(())
}