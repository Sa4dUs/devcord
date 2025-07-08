use std::{env::var, net::SocketAddr, sync::Arc};

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
    api_utils::structs::PrivateUser,
    fluvio_consumer,
    request::{
        block::{block_user, get_blocked, unblock_user},
        friendships::{
            accept_friend, get_friends, get_request_recieved, get_request_sent, reject_friend,
            request_friend,
        },
        user::{get_user_info, update_profile},
    },
    sql_utils::calls::insert_user,
};

#[derive(Clone)]
pub struct AppState {
    pub db: sqlx::PgPool,
    pub producer: TopicProducer<SpuSocketPool>,
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

    //ONLY FOR TESTING

    let user_a = PrivateUser {
        id: "a".to_owned(),
        username: "a_username".to_owned(),
        created_at: None,
    };

    let user_b = PrivateUser {
        id: "b".to_owned(),
        username: "b_username".to_owned(),
        created_at: None,
    };

    insert_user(user_a, &db).await.ok();
    insert_user(user_b, &db).await.ok();

    //NO LONGER FOR TESTING

    let state = Arc::new(AppState {
        db: db.clone(),
        producer,
    });

    let friendships_router = Router::new()
        .route("/request", post(request_friend))
        .route("/accept", post(accept_friend))
        .route("/reject", post(reject_friend))
        .route("/sent", get(get_request_sent))
        .route("/recieved", get(get_request_recieved))
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
