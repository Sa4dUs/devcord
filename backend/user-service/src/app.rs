use std::{env::var, net::SocketAddr, sync::Arc, time::Duration};

use axum::{
    Router,
    http::{HeaderValue, Method, header},
    routing::{get, post},
    serve,
};
use fluvio::{Fluvio, FluvioConfig, TopicProducer, metadata::topic::TopicSpec, spu::SpuSocketPool};
use sqlx::postgres::PgPoolOptions;
use tokio::net::TcpListener;
use tower_http::{cors::CorsLayer, trace::TraceLayer};
use tracing::Level;
use tracing_subscriber::{Layer, filter, layer::SubscriberExt, util::SubscriberInitExt};

use crate::{
    fluvio_consumer,
    request::{
        block::{block_user, get_blocked, unblock_user},
        friendships::{
            accept_friend, get_friends, get_request_received, get_request_sent, reject_friend,
            request_friend,
        },
        user::{get_user_info, update_profile},
    },
    sql_utils::init::init,
};

#[derive(Clone)]
pub struct AppState {
    pub db: sqlx::PgPool,
    pub request_sent_producer: TopicProducer<SpuSocketPool>,
    pub request_answered_producer: TopicProducer<SpuSocketPool>,
}

pub async fn app() -> anyhow::Result<(Router, Fluvio, sqlx::PgPool)> {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::fmt::layer()
                .with_filter(filter::LevelFilter::from_level(Level::DEBUG)),
        )
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

    let trace_layer = TraceLayer::new_for_http();

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
            var("DATABASE_URL")
                .expect("DATABASE_URL env not set")
                .trim(),
        )
        .await?;

    init(&db).await?;

    let mut fluvio_config =
        FluvioConfig::new(var("FLUVIO_ADDR").expect("FLUVIO_ADDR env not set").trim());
    fluvio_config.use_spu_local_address = true;

    let fluvio = fluvio::Fluvio::connect_with_config(&fluvio_config).await?;

    let auth_registered_consumer_topic = var("AUTH_REGISTER_TOPIC")
        .unwrap_or("auth-register".to_owned())
        .trim()
        .to_string();

    let request_producer_topic = var("USER_RESQUEST_TOPIC")
        .unwrap_or("friendships-request".to_owned())
        .trim()
        .to_string();

    let answered_producer_topic = var("USER_ANSWER_TOPIC")
        .unwrap_or("friendships-answer".to_owned())
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

    //Creates topic if they dont exist

    if !topic_names.contains(&request_producer_topic) {
        let topic_spec = TopicSpec::new_computed(1, 1, None);
        admin
            .create(request_producer_topic.clone(), false, topic_spec)
            .await?;
    }

    if !topic_names.contains(&answered_producer_topic) {
        let topic_spec = TopicSpec::new_computed(1, 1, None);
        admin
            .create(answered_producer_topic.clone(), false, topic_spec)
            .await?;
    }

    if !topic_names.contains(&auth_registered_consumer_topic) {
        let topic_spec = TopicSpec::new_computed(1, 1, None);
        admin
            .create(auth_registered_consumer_topic.clone(), false, topic_spec)
            .await?;
    }

    let request_producer = fluvio.topic_producer(request_producer_topic).await?;

    let answered_producer = fluvio.topic_producer(answered_producer_topic).await?;

    let state = Arc::new(AppState {
        db: db.clone(),
        request_sent_producer: request_producer,
        request_answered_producer: answered_producer,
    });

    let friendships_router = Router::new()
        .route("/request", post(request_friend))
        .route("/accept", post(accept_friend))
        .route("/reject", post(reject_friend))
        .route("/sent", get(get_request_sent))
        .route("/received", get(get_request_received))
        .route("/friends", get(get_friends));

    let block_router = Router::new()
        .route("/block", post(block_user))
        .route("/unblock", post(unblock_user))
        .route("/", get(get_blocked));

    let app = Router::new()
        .nest("/friendship", friendships_router)
        .nest("/blocks", block_router)
        .route("/update", post(update_profile))
        .route("/", get(get_user_info))
        .route(
            "/health",
            get(|| async { "Long life to the allmighty turbofish" }),
        )
        .layer(cors_layer)
        .layer(trace_layer)
        .with_state(state);

    Ok((app, fluvio, db))
}

pub async fn run() -> anyhow::Result<()> {
    let (app, fluvio, db) = app().await?;

    let addr: SocketAddr = var("SOCKET_ADDR")
        .expect("SOCKET_ADDR env not set")
        .parse()?;
    let listener = TcpListener::bind(addr).await?;

    println!("Server runnnig at: {addr}");

    let consumer_thread = tokio::spawn(fluvio_consumer::run(fluvio, db));

    serve(listener, app.into_make_service()).await?;
    consumer_thread.await??;
    Ok(())
}
