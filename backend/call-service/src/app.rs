use std::{env::var, sync::Arc};

use axum::{
    Router,
    http::{HeaderValue, Method, header},
    routing::get,
};
use dashmap::DashMap;
use dotenv::dotenv;
use tokio::net::TcpListener;
use tower_http::{cors::CorsLayer, trace::TraceLayer};
use tracing::info;
use tracing_subscriber::{EnvFilter, fmt, layer::SubscriberExt, util::SubscriberInitExt};

use crate::{room::Room, user_ws::handle_upgrade};

#[derive(Default, Clone)]
pub struct AppState {
    pub rooms: Arc<DashMap<String, Room>>,
}

pub fn app() -> anyhow::Result<Router> {
    dotenv().ok();
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

    let app_state = Arc::new(AppState::default());

    let app = Router::new()
        .route("/health", get(|| async { "Viva el imperio Mongol!" }))
        .route("/", get(handle_upgrade))
        .layer(trace_layer)
        .layer(cors_layer)
        .with_state(app_state);

    Ok(app)
}

pub async fn run(app: Router) -> anyhow::Result<()> {
    let listener = TcpListener::bind("0.0.0.0:3000").await.unwrap();

    info!("Server running on 0.0.0.0:3000");

    axum::serve(listener, app).await?;

    Ok(())
}
