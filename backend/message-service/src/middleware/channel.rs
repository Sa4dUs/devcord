use axum::body::to_bytes;
use axum::{
    Extension, Json,
    body::Body,
    extract::Request,
    http::{StatusCode, header},
    middleware::Next,
    response::{IntoResponse, Response},
};
use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct Channel {
    pub id: String,
}

#[derive(Debug, Deserialize)]
struct BodyChannelId {
    channel_id: Option<String>,
}

pub async fn extract_channel(mut req: Request<Body>, next: Next) -> Result<Response, ChannelError> {
    // FIXME(Sa4dUs): Collapse these `if let` statements once `Dockerfile` runs rust 2024 edition
    if let Some(header_value) = req.headers().get(header::SEC_WEBSOCKET_PROTOCOL) {
        if let Ok(protocols_str) = header_value.to_str() {
            if let Some(protocol) = protocols_str.split(',').map(str::trim).nth(1) {
                let channel = Channel {
                    id: protocol.to_string(),
                };
                req.extensions_mut().insert(channel);
                return Ok(next.run(req).await);
            }
        }
    }

    let (parts, body) = req.into_parts();
    let body_bytes = axum::body::to_bytes(body, usize::MAX)
        .await
        .map_err(|_| ChannelError::MissingChannel)?;

    // FIXME(Sa4dUs): Collapse these `if let` statements once `Dockerfile` runs rust 2024 edition
    if let Some(path) = parts.uri.path().split('/').nth(1) {
        if !path.is_empty() {
            let channel = Channel {
                id: path.to_string(),
            };
            let mut req = Request::from_parts(parts, Body::from(body_bytes));
            req.extensions_mut().insert(channel);
            return Ok(next.run(req).await);
        }
    }

    let maybe_json: Result<BodyChannelId, _> = serde_json::from_slice(&body_bytes);

    if let Ok(BodyChannelId {
        channel_id: Some(id),
    }) = maybe_json
    {
        let mut req = Request::from_parts(parts, Body::from(body_bytes));
        req.extensions_mut().insert(Channel { id });
        return Ok(next.run(req).await);
    }

    Err(ChannelError::MissingChannel)
}

#[allow(dead_code)]
#[derive(Debug)]
pub enum ChannelError {
    WrongChannel,
    MissingChannel,
}

impl IntoResponse for ChannelError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            ChannelError::WrongChannel => (StatusCode::BAD_REQUEST, "Wrong channel_id"),
            ChannelError::MissingChannel => (StatusCode::BAD_REQUEST, "Missing channel_id"),
        };
        let body = Json(json!({
            "error": error_message,
        }));
        (status, body).into_response()
    }
}
