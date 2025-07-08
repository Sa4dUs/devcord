use crate::log_out::log_user_out;
use crate::models::app_state::AppState;
use crate::register::register_user;
use crate::sign_in::sign_in_user;

use anyhow::Result;
use axum::{Router, routing::post};
use dotenvy::dotenv;
use fluvio::Fluvio;
use sqlx::postgres::PgPoolOptions;
use std::{env, time::Duration};
use tower_http::trace::TraceLayer;

use tracing::info;

pub async fn run() -> Result<()> {
    dotenv().ok();

    let origins: Vec<HeaderValue> = var("CORS_ORIGIN")
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

    let database_url = env::var("AUTH_DATABASE_URL")
        .map_err(|_| anyhow::anyhow!("AUTH_DATABASE_URL must be set in .env"))?;

    let max_conns: u32 = env::var("DB_MAX_CONNECTIONS")
        .expect("DB_MAX_CONNECTIONS must be set")
        .parse()
        .expect("DB_MAX_CONNECTIONS must be a number");

    let db_timeout: u64 = env::var("DB_POOL_TIMEOUT_SECS")
        .expect("DB_POOL_TIMEOUT_SECS must be set")
        .parse()
        .expect("DB_POOL_TIMEOUT_SECS must be a number");

    let pool = PgPoolOptions::new()
        .max_connections(max_conns)
        .acquire_timeout(Duration::from_secs(db_timeout))
        .connect(&database_url)
        .await?;

    sqlx::migrate!().run(&pool).await?;
    info!("Database connected");

    let fluvio = Fluvio::connect().await?;
    let producer = fluvio.topic_producer("auth").await?;
    info!("Connected to Fluvio");

    let state = AppState { db: pool, producer };

    let app = Router::new()
        .route("/register", post(register_user))
        .route("/sign_in", post(sign_in_user))
        .route("/log_out", post(log_user_out))
        .layer(cors_layer)
        .layer(trace_layer)
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;
    info!("Server listening on 0.0.0.0:3000");

    axum::serve(listener, app).await?;

    Ok(())
}
