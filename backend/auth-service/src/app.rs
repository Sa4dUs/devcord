use crate::db::init::init;
use crate::log_out::log_user_out;
use crate::models::app_state::AppState;
use crate::register::register_user;
use crate::sign_in::sign_in_user;

use anyhow::Result;
use axum::http::{HeaderValue, Method, header};
use axum::{Router, routing::post};
use dotenvy::dotenv;
use fluvio::FluvioConfig;
use fluvio::metadata::topic::TopicSpec;
use sqlx::postgres::PgPoolOptions;
use std::env::var;
use std::{env, time::Duration};
use tower_http::cors::CorsLayer;
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
        .allow_headers([
            header::CONTENT_TYPE,
            header::AUTHORIZATION,
            header::ACCESS_CONTROL_ALLOW_ORIGIN,
        ]);

    let trace_layer = TraceLayer::new_for_http();

    let database_url = env::var("AUTH_DATABASE_URL")
        .map_err(|_| anyhow::anyhow!("AUTH_DATABASE_URL must be set in .env"))?;

    let max_conns: u32 = env::var("DB_MAX_CONNECTIONS")
        .unwrap_or("1".to_owned())
        .parse()
        .expect("DB_MAX_CONNECTIONS must be a number");

    let db_timeout: u64 = env::var("DB_POOL_TIMEOUT_SECS")
        .unwrap_or("10".to_owned())
        .parse()
        .expect("DB_POOL_TIMEOUT_SECS must be a number");

    let pool = PgPoolOptions::new()
        .max_connections(max_conns)
        .acquire_timeout(Duration::from_secs(db_timeout))
        .connect(&database_url)
        .await?;

    init(&pool).await?;
    info!("Database connected");

    let mut fluvio_config =
        FluvioConfig::new(var("FLUVIO_ADDR").expect("FLUVIO_ADDR env not set").trim());
    fluvio_config.use_spu_local_address = true;

    let fluvio = fluvio::Fluvio::connect_with_config(&fluvio_config).await?;

    let register_topic = var("AUTH_REGISTER_TOPIC")
        .unwrap_or("auth-register".to_owned())
        .trim()
        .to_string();
    let login_topic = var("AUTH_LOGIN_TOPIC")
        .unwrap_or("auth-login".to_owned())
        .trim()
        .to_string();
    let logout_topic = var("AUTH_LOGOUT_TOPIC")
        .unwrap_or("auth-logout".to_owned())
        .trim()
        .to_string();

    let admin = fluvio.admin().await;

    let topics = admin
        .all::<TopicSpec>()
        .await
        .expect("Failed to list topics");
    let topic_names = topics
        .iter()
        .map(|topic| topic.name.clone())
        .collect::<Vec<String>>();

    if !topic_names.contains(&register_topic) {
        let topic_spec = TopicSpec::new_computed(1, 1, None);
        admin
            .create(register_topic.clone(), false, topic_spec)
            .await?;
    }

    if !topic_names.contains(&login_topic) {
        let topic_spec = TopicSpec::new_computed(1, 1, None);
        admin.create(login_topic.clone(), false, topic_spec).await?;
    }

    if !topic_names.contains(&logout_topic) {
        let topic_spec = TopicSpec::new_computed(1, 1, None);
        admin
            .create(logout_topic.clone(), false, topic_spec)
            .await?;
    }

    let register_producer = fluvio.topic_producer(register_topic).await?;
    let login_producer = fluvio.topic_producer(login_topic).await?;
    let logout_producer = fluvio.topic_producer(logout_topic).await?;
    info!("Connected to Fluvio");

    let state = AppState {
        db: pool,
        register_producer,
        login_producer,
        logout_producer,
    };

    let app = Router::new()
        .route("/register", post(register_user))
        .route("/login", post(sign_in_user))
        .route("/logout", post(log_user_out))
        .layer(cors_layer)
        .layer(trace_layer)
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;
    info!("Server listening on 0.0.0.0:3000");

    axum::serve(listener, app).await?;

    Ok(())
}
