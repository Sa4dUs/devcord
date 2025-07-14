use std::{env::var, net::SocketAddr, sync::Arc};

use axum::{
    Router,
    extract::ws::Message,
    http::{HeaderValue, Method, header},
    routing::get,
    serve,
};
use dashmap::DashMap;
use tokio::{
    net::TcpListener,
    sync::mpsc::{Receiver, Sender},
};
use tower_http::{cors::CorsLayer, trace::TraceLayer};
use tracing_subscriber::{EnvFilter, fmt, layer::SubscriberExt, util::SubscriberInitExt};

use crate::{connection::notification_handler, fluvio_reader};

pub type ResponseSender = Sender<Message>;
pub type ResponseReceiver = Receiver<Message>;

#[derive(Clone, Default)]
pub struct AppState {
    pub channels: Arc<DashMap<String, ResponseSender>>,
}

pub async fn app() -> anyhow::Result<Router> {
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

    tracing_subscriber::registry()
        .with(fmt::layer())
        .with(EnvFilter::from_default_env())
        .init();

    let trace_layer = TraceLayer::new_for_http();

    let state = Arc::new(AppState::default());

    let addr = var("FLUVIO_ADDR")
        .expect("FLUVIO_ADDR env not set")
        .trim()
        .to_string();
    let channels = state.channels.clone();

    let mut handles = Vec::default();

    let channels_c = channels.clone();
    let addr_c = addr.clone();
    handles.push(tokio::spawn(fluvio_reader::run::<
        topic_structs::FriendRequestCreated,
    >(
        channels_c,
        addr_c,
        "USER_RESQUEST_TOPIC",
        "friendship_requested",
    )));
    let channels_c = channels.clone();
    let addr_c = addr.clone();
    handles.push(tokio::spawn(fluvio_reader::run::<
        topic_structs::FriendRequestAnswered,
    >(
        channels_c,
        addr_c,
        "USER_ANSWER_TOPIC",
        "friendship_answered",
    )));

    let router: Router = Router::new()
        .route("/", get(notification_handler))
        .route("/health", get(|| async { "Healthy :D" }))
        .with_state(state)
        .layer(cors_layer)
        .layer(trace_layer);

    Ok(router)
}

pub async fn run() -> anyhow::Result<()> {
    let app = app().await?;

    let addr: SocketAddr = var("SOCKET_ADDR")
        .expect("OCKET_ADDR env not set")
        .parse()?;

    let listener = TcpListener::bind(addr).await?;

    println!("Server starting at: {addr}");

    serve(listener, app.into_make_service()).await?;

    Ok(())
}
