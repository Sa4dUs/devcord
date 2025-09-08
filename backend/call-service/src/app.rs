use std::{env::var, sync::Arc, time::Duration};

use anyhow::Result;
use axum::{
    Router,
    http::{HeaderValue, Method, header},
    routing::get,
};
use sqlx::postgres::PgPoolOptions;
use tokio::{net::TcpListener, sync::mpsc};
use tower_http::{cors::CorsLayer, trace::TraceLayer};
use tracing::{error, info};
use tracing_subscriber::{EnvFilter, fmt, layer::SubscriberExt, util::SubscriberInitExt};

use crate::{
    appstate::{AppState, AppStateMessage},
    user_ws::handle_upgrade,
};

pub async fn app() -> anyhow::Result<(Router, mpsc::Sender<AppStateMessage>)> {
    let trace_layer = TraceLayer::new_for_http();

    tracing_subscriber::registry()
        .with(fmt::layer())
        .with(EnvFilter::from_default_env())
        .init();

    let origins: Vec<HeaderValue> = var("CORS_ORIGIN")
        .expect("CORS_ORIGIN env not set")
        .split(",")
        .map(|e| e.trim().parse::<HeaderValue>())
        .collect::<Result<_, _>>()?;

    let cors_layer = CorsLayer::new()
        .allow_origin(origins)
        .allow_credentials(true)
        .allow_methods([Method::GET, Method::POST])
        .allow_headers([
            header::CONTENT_TYPE,
            header::AUTHORIZATION,
            header::ACCESS_CONTROL_ALLOW_ORIGIN,
        ]);

    let max_conns: u32 = var("DB_MAX_CONNECTIONS")
        .unwrap_or("1".to_owned())
        .parse()
        .expect("DB_MAX_CONNECTIONS must be a number");

    let db_timeout: u64 = var("DB_POOL_TIMEOUT_SECS")
        .unwrap_or("10".to_owned())
        .parse()
        .expect("DB_POOL_TIMEOUT_SECS must be a number");

    let db = PgPoolOptions::new()
        .max_connections(max_conns)
        .acquire_timeout(Duration::from_secs(db_timeout))
        .connect(
            var("CALL_DATABASE_URL")
                .expect("CALL_DATABASE_URL env not set")
                .trim(),
        )
        .await
        .map_err(|e| {
            error!("Error conecting to db: {}", e);
            e
        })?;

    crate::sql_utils::init::run(&db).await?;

    let app_state = Arc::new(AppState::new(db).await?);
    let sender = app_state.sender.clone();
    let app = Router::new()
        .route("/health", get(|| async { "Healthy" }))
        .route("/", get(handle_upgrade))
        .layer(trace_layer)
        .layer(cors_layer)
        .with_state(app_state);

    Ok((app, sender))
}

pub async fn run(app: Router, sender: mpsc::Sender<AppStateMessage>) -> anyhow::Result<()> {
    let listener = TcpListener::bind("0.0.0.0:3000").await?;

    info!("Server running on 0.0.0.0:3000");

    axum::serve(listener, app)
        .await
        .map_err(|e| error!("App crashed: {}", e));

    sender.send(AppStateMessage::AppClosed).await?;
    Ok(())
}
