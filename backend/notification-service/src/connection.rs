use std::sync::Arc;

use axum::{
    extract::{State, WebSocketUpgrade, ws::WebSocket},
    response::IntoResponse,
};
use tokio::sync::mpsc::{self};

use crate::{
    app::{AppState, ResponseReceiver},
    jwt::Claims,
};

pub async fn notification_handler(
    ws: WebSocketUpgrade,
    State(state): State<Arc<AppState>>,
    claims: Claims,
) -> impl IntoResponse {
    let (sender, receiver) = mpsc::channel(10);

    state.channels.insert(claims.user_id.clone(), sender);

    ws.on_upgrade(move |socket| handle_notifications(socket, receiver, state, claims))
}

pub async fn handle_notifications(
    mut socket: WebSocket,
    mut receiver: ResponseReceiver,
    state: Arc<AppState>,
    claims: Claims,
) {
    while let Some(msg) = receiver.recv().await {
        let Ok(_) = socket.send(msg).await else {
            break;
        };
    }

    state.channels.remove(&claims.user_id);
}
