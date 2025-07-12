use std::{env::var, net::SocketAddr, sync::Arc};

use axum::{Router, extract::ws::Message, routing::get, serve};
use dashmap::DashMap;
use tokio::{
    net::TcpListener,
    sync::mpsc::{Receiver, Sender},
};

use crate::connection::notification_handler;

pub type ResponseSender = Sender<Message>;
pub type ResponseReceiver = Receiver<Message>;

#[derive(Clone, Default)]
pub struct AppState {
    pub channels: Arc<DashMap<String, ResponseSender>>,
}

pub async fn app() -> anyhow::Result<Router> {
    let state = Arc::new(AppState::default());

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
