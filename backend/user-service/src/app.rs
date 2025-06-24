use std::{env::var, net::SocketAddr, sync::Arc};

use axum::{
    Router,
    http::{HeaderValue, Method, header},
    routing::post,
    serve,
};
use fluvio::{FluvioConfig, metadata::topic::TopicSpec};
use sqlx::postgres::PgPoolOptions;
use tokio::net::TcpListener;
use tower_http::{cors::CorsLayer, trace::TraceLayer};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use crate::{answer_request::answer, fluvio_consumer, request::request, update::update};

#[derive(Clone)]
pub struct AppState {
    pub db: sqlx::PgPool,
    pub producer: fluvio::TopicProducer<fluvio::spu::SpuSocketPool>,
}

pub async fn run() -> anyhow::Result<()> {
    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer())
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
        .allow_headers([header::CONTENT_TYPE, header::AUTHORIZATION]);

    let trace_layer = TraceLayer::new_for_http();

    let db = PgPoolOptions::new()
        .connect(
            var("DATABASE_URL")
                .expect("DATABASE_URL env not set")
                .trim(),
        )
        .await?;

    sqlx::migrate!().run(&db).await?;

    let mut fluvio_config =
        FluvioConfig::new(var("FLUVIO_ADDR").expect("FLUVIO_ADDR env not set").trim());
    fluvio_config.use_spu_local_address = true;

    let fluvio = fluvio::Fluvio::connect_with_config(&fluvio_config).await?;

    let producer_topic = var("PRODUCER_TOPIC")
        .expect("PRODUCER_TOPIC env not set")
        .trim()
        .to_string();

    let consumer_topic = var("CONSUMER_TOPIC")
        .expect("CONSUMER_TOPIC env not set")
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

    if !topic_names.contains(&producer_topic) {
        let topic_spec = TopicSpec::new_computed(1, 1, None);
        admin
            .create(producer_topic.clone(), false, topic_spec)
            .await?;
    }

    if !topic_names.contains(&consumer_topic) {
        let topic_spec = TopicSpec::new_computed(1, 1, None);
        admin
            .create(consumer_topic.clone(), false, topic_spec)
            .await?;
    }

    let producer = fluvio.topic_producer(producer_topic).await?;

    let state = Arc::new(AppState {
        db: db.clone(),
        producer,
    });

    let app = Router::new()
        .route("/update", post(update))
        .route("/request", post(request))
        .route("/answer", post(answer))
        .layer(cors_layer)
        .layer(trace_layer)
        .with_state(state);

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
