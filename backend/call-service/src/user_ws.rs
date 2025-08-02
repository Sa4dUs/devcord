use std::sync::Arc;

use anyhow::{Result, anyhow};
use axum::{
    extract::{
        State, WebSocketUpgrade,
        ws::{Message, WebSocket},
    },
    response::IntoResponse,
};
use futures_util::{SinkExt, StreamExt};
use tracing::debug;

use crate::{
    app::AppState,
    jwt::{Authenticated, Claims},
    room::WSFromUserMessage,
};

pub async fn handle_upgrade(
    ws: WebSocketUpgrade,
    State(state): State<Arc<AppState>>,
    Authenticated { claims, jwt }: Authenticated,
) -> impl IntoResponse {
    debug!("WS Received, id {}", claims.user_id);
    ws.protocols([jwt])
        .on_upgrade(move |ws| handle_ws(ws, state, claims))
}

async fn handle_ws(ws: WebSocket, state: Arc<AppState>, claims: Claims) {
    let (mut ws_tx, mut ws_rx) = ws.split();

    debug!("Waiting for room id");
    let WSFromUserMessage::ConnectToRoom { room_id } =
        //TODO! ...
        parse_msg(ws_rx.next().await.unwrap().unwrap()).unwrap() 
    else {
        debug!("Room id parsing error");
        return; //TODO! Do this properly
    };

    debug!("Room id valid: {}", room_id);

    let room = state.get_room(&room_id).await.unwrap();
    //  else {
    //     debug!("Room returning error");
    //     return; //Handle error
    // };

    debug!("Room returned");

    let mut room_lock = room.lock().await;

    let (room_tx, mut room_rx) = room_lock.new_user(&claims.user_id).await.unwrap(); //TODO! Error if not allowed, need to do something

    tokio::spawn(async move {
        while let Some(msg) = room_rx.recv().await {
            match msg {
                crate::room::WSInnerUserMessage::Message(msg) => ws_tx.send(msg).await.unwrap(), //If ws closed we remove thread, but properly
                crate::room::WSInnerUserMessage::Close => {
                    ws_tx.close().await.unwrap(); //TODO! WTF why this error?? xd
                    break;
                }
            }
        }
    });

    tokio::spawn(async move {
        while let Some(Ok(msg)) = ws_rx.next().await {
            //TODO! Probs should do this better :D
            let Ok(msg) = parse_msg(msg) else {
                continue; //TODO! Proper error response handling
            };

            room_tx.send(msg).await.unwrap(); //TODO! :D
        }
    });
}

fn parse_msg(msg: Message) -> Result<WSFromUserMessage> {
    let msg_str = match msg {
        Message::Text(msg) => msg,
        e => return Err(anyhow!("Recieved msg is not valid: {:?}", e)),
    };

    let msg_str = msg_str.as_str().trim();

    let msg = serde_json::from_str::<WSFromUserMessage>(msg_str)
        .map_err(|e| anyhow!("Recieved msg is not valid: {e}"))?;

    Ok(msg)
}
