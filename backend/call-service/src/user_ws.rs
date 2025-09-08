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
use tracing::{debug, error};

use crate::{
    appstate::AppState,
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

    //Maybe change this so it can loop, rn its good tho
    debug!("Waiting for room id");
    let WSFromUserMessage::ConnectToRoom { room_id } = ({
        let Ok(msg) = ws_rx
            .next()
            .await
            .unwrap() //FIXME! Since idk when the ws send none im just ignoring it, shuld check at some point but hasnt crashed it yet
            .map_err(|e| error!("Error listening to user web socket {}", e))
        else {
            ws_tx.close(); //FIXME! Give feedback
            return;
        };
        let Ok(msg) = parse_msg(msg).map_err(|e| error!("Room id parsing error {}", e)) else {
            ws_tx.close(); //FIXME! Give feedback
            return;
        };
        msg
    }) else {
        error!("User sent a non connect to room msg");
        ws_tx.close(); //FIXME! Give feedback
        return;
    };

    debug!("Room id valid: {}", room_id);

    let Ok(room) = state.get_room(&room_id).await.map_err(|e| {
        debug!(
            "Could not retrieve room {} from database due to {}",
            room_id, e
        );
    }) else {
        ws_tx.close(); //FIXME! Give feedback
        return;
    };

    debug!("Room fetched: {:?}", room);

    let mut room_lock = room.lock().await;

    let Ok((room_tx, mut room_rx)) = room_lock
        .new_user(&claims.user_id)
        .await
        .map_err(|e| debug!("Error when user tried to join the room: {}", e))
    else {
        ws_tx.close(); //FIXME! Give feedback
        return;
    };

    tokio::spawn(async move {
        while let Some(msg) = room_rx.recv().await {
            if let Err(e) = match msg {
                crate::room::WSInnerUserMessage::Message(msg) => ws_tx.send(msg).await,
                crate::room::WSInnerUserMessage::Close => {
                    ws_tx.close(); //FIXME! Give feedback
                    break;
                }
            } {
                debug!("Error sending ws msg: {}", e);
                //We asume its closed
                return;
            }
        }
    });

    tokio::spawn(async move {
        while let Some(Ok(msg)) = ws_rx.next().await {
            let Ok(msg) = parse_msg(msg).map_err(|e| debug!("Error parsing user msg: {}", e))
            else {
                continue; //FIXME! Give feedback
            };

            if let Err(e) = room_tx.send(msg).await {
                if room_tx.is_closed() {
                    return;
                }
                error!("Error comunicating from ws to room: {}", e);
                break;
            }
        }

        if room_tx.is_closed() {
            return;
        }

        let msg = WSFromUserMessage::Disconected {
            user_id: Arc::new(claims.user_id),
        };
        if let Err(e) = room_tx.send(msg).await {
            error!("Error comunicating disconection from ws to room: {}", e);
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
