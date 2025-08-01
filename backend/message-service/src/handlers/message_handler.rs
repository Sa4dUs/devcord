use std::sync::Arc;

use axum::{
    Extension,
    extract::{State, WebSocketUpgrade, ws::WebSocket},
    response::IntoResponse,
};

use crate::{middleware::auth::Authenticated, models::claims::Claims, state::AppState};

pub async fn message_handler(
    State(state): State<Arc<AppState>>,
    ws: WebSocketUpgrade,
    Extension(Authenticated { claims, jwt }): Extension<Authenticated>,
) -> impl IntoResponse {
    ws.protocols([jwt])
        .on_upgrade(move |socket| handle_messages(socket, state, claims))
}

pub async fn handle_messages(mut socket: WebSocket, state: Arc<AppState>, claims: Claims) {
    while let Some(Ok(msg)) = socket.recv().await {
        let Ok(_) = socket.send(msg).await else {
            break;
        };
    }
}
