use std::{env::var, net::SocketAddr, sync::Arc};

use axum::{Router, extract::ws::Message, routing::get, serve};
use dashmap::DashMap;
use fluvio::{Fluvio, FluvioConfig};
use tokio::{
    net::TcpListener,
    sync::mpsc::{Receiver, Sender},
};

use crate::connection::notification_handler;

pub type ResponseSender = Sender<Message>;
pub type ResponseReceiver = Receiver<Message>;

#[derive(Clone)]
pub struct AppState {
    pub channels: Arc<DashMap<String, ResponseSender>>,
    pub fluvio: Arc<Fluvio>,
}

pub async fn app() -> anyhow::Result<Router> {
    let mut fluvio_config =
        FluvioConfig::new(var("FLUVIO_ADDR").expect("FLUVIO_ADDR env not set").trim());
    fluvio_config.use_spu_local_address = true;

    let fluvio = fluvio::Fluvio::connect_with_config(&fluvio_config).await?;

    let state = Arc::new(AppState {
        channels: Arc::default(),
        fluvio: Arc::new(fluvio),
    });

    let channels = state.channels.clone();

    let router: Router = Router::new()
        .route("/", get(notification_handler))
        .route("/health", get(|| async { "Healthy :D" }))
        .with_state(state);

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
