use std::env;
use std::sync::Arc;

use axum::Json;
use axum::extract::Query;
use axum::extract::ws::Message;
use axum::{
    Extension,
    extract::{State, WebSocketUpgrade, ws::WebSocket},
    response::IntoResponse,
};
use fluvio::metadata::core::Status;
use reqwest::{Client, StatusCode};
use serde::Deserialize;
use topic_structs::MessageSent;
use uuid::Uuid;

use crate::middleware::channel;
use crate::models::group::{GroupIdResponse, GroupMembersResponse};
use crate::models::message::MessageInfo;
use crate::{
    middleware::{auth::Authenticated, channel::Channel},
    models::claims::Claims,
    state::AppState,
};

#[derive(Deserialize)]
pub struct MessageQueryParams {
    pub from: Option<usize>,
    pub to: Option<usize>,
}

pub async fn message_handler(
    State(state): State<Arc<AppState>>,
    ws: WebSocketUpgrade,
    Extension(Authenticated { claims, jwt }): Extension<Authenticated>,
    Extension(Channel { id }): Extension<Channel>,
) -> impl IntoResponse {
    ws.protocols([jwt.clone()])
        .on_upgrade(move |socket| handle_messages(socket, state, claims, id, jwt))
}

async fn handle_messages(
    mut socket: WebSocket,
    state: Arc<AppState>,
    claims: Claims,
    channel_id: String,
    jwt: String,
) {
    let Ok(sender_id) = Uuid::parse_str(&claims.user_id) else {
        return;
    };

    let Ok(channel_id) = Uuid::parse_str(&channel_id) else {
        return;
    };

    let members: Vec<Uuid> = match fetch_group_members(channel_id, jwt).await {
        Ok(list) => list,
        Err(e) => {
            tracing::error!("Failed to get member list: {e}");
            return;
        }
    };

    while let Some(Ok(msg)) = socket.recv().await {
        let message = match msg {
            Message::Text(text) => text,
            _ => break,
        };

        let event = MessageSent {
            channel_id: channel_id.to_string(),
            sender: sender_id.to_string(),
            message: message.to_string(),
        };

        let Ok(event_bytes) = serde_json::to_vec(&event) else {
            tracing::error!("Failed to serialize `MessageSent` event");
            break;
        };

        for user_id in &members {
            if let Err(e) = state
                .producer
                .send(user_id.to_string(), event_bytes.clone())
                .await
            {
                tracing::error!("Failed to send to user {}: {e}", user_id);
            }
        }

        let message_id = Uuid::new_v4();

        if let Err(e) = sqlx::query(
            "INSERT INTO messages (id, sender_id, channel_id, message) VALUES ($1, $2, $3, $4)",
        )
        .bind(message_id)
        .bind(sender_id)
        .bind(channel_id)
        .bind(message.to_string())
        .execute(&state.db)
        .await
        {
            tracing::error!("Could not insert into database: {e}");
            break;
        }

        // echo just for confirmation purposes
        if socket.send(Message::Text(message)).await.is_err() {
            break;
        }
    }
}

async fn fetch_group_members(channel_id: Uuid, jwt: String) -> Result<Vec<Uuid>, reqwest::Error> {
    let group_service_url =
        env::var("GROUP_SERVICE_URL").expect("GROUP_SERVICE_URL env variable must be set");

    let client = Client::new();

    let group_id_url = format!("{group_service_url}/by_channel/{channel_id}");
    let res = client
        .get(&group_id_url)
        .header("Authorization", format!("Bearer {jwt}"))
        .send()
        .await?;

    let group_id: Uuid = res.json().await?;

    let members_url = format!("{group_service_url}/{group_id}/members");
    let res = client
        .get(&members_url)
        .header("Authorization", format!("Bearer {jwt}"))
        .send()
        .await?;

    let members: Vec<Uuid> = res.json().await?;

    Ok(members)
}

pub async fn fetch_messages(
    State(state): State<Arc<AppState>>,
    Extension(Authenticated { claims, jwt }): Extension<Authenticated>,
    Extension(Channel { id: channel_id }): Extension<Channel>,
    Query(params): Query<MessageQueryParams>,
) -> Result<impl IntoResponse, StatusCode> {
    let (offset, limit) = match (params.from, params.to) {
        (Some(from), Some(to)) if to > from => (from as i64, (to - from) as i64),
        (Some(from), None) => (from as i64, 10),
        (None, Some(to)) => (0, to as i64),
        _ => (0, 10),
    };

    let Ok(channel_id) = Uuid::parse_str(&channel_id) else {
        tracing::error!("error channel_id");
        return Err(StatusCode::INTERNAL_SERVER_ERROR);
    };

    let Ok(group_members) = fetch_group_members(channel_id, jwt).await else {
        tracing::error!("error group_members");
        return Err(StatusCode::INTERNAL_SERVER_ERROR);
    };

    tracing::info!("{} {:?}", claims.user_id, group_members);
    if !group_members.contains(&Uuid::parse_str(&claims.user_id).unwrap()) {
        return Err(StatusCode::NOT_FOUND);
    }

    let messages = sqlx::query_as::<_, MessageInfo>(
        "SELECT m.id, m.sender_id, m.channel_id, m.message, m.created_at FROM messages m
          WHERE m.channel_id = $1
          ORDER BY m.created_at DESC
          OFFSET $2 LIMIT $3",
    )
    .bind(channel_id)
    .bind(offset)
    .bind(limit)
    .fetch_all(&state.db)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(messages))
}
