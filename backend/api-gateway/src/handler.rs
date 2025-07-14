use axum::body::to_bytes;
use axum::extract::WebSocketUpgrade;
use axum::extract::ws::{CloseFrame, Message, Utf8Bytes, WebSocket};
use axum::{
    Extension,
    body::Body,
    extract::Request,
    http::{StatusCode, Uri},
    response::Response,
};
use futures_util::{SinkExt, StreamExt};
use hyper::header::CONTENT_LENGTH;

use reqwest::Client;
use tokio_tungstenite::connect_async;

use crate::config::{Instance, Service};
use crate::error::ErrorResponse;
use crate::middleware::parser::ParsedURI;

pub(crate) async fn http_handler(
    Extension(ParsedURI { prefix: _, subpath }): Extension<ParsedURI>,
    Extension(_): Extension<Service>,
    Extension(Instance(base_uri)): Extension<Instance>,
    mut req: Request,
) -> Result<Response<Body>, StatusCode> {
    let original_uri = req.uri();
    let query = original_uri
        .query()
        .map(|q| format!("?{q}"))
        .unwrap_or_default();

    let uri_str = format!("http://{base_uri}{subpath}{query}");
    let uri: Uri = uri_str
        .parse()
        .map_err(|_| StatusCode::BAD_GATEWAY.with_debug("Invalid uri. Could not parse"))?;
    *req.uri_mut() = uri;

    let (parts, body) = req.into_parts();

    let content_length = parts
        .headers
        .get(CONTENT_LENGTH)
        .and_then(|val| val.to_str().ok()?.parse::<usize>().ok())
        .unwrap_or(0);

    let body_bytes = to_bytes(body, content_length).await.map_err(|_| {
        StatusCode::BAD_GATEWAY.with_debug("Invalid request body. Could not convert to bytes")
    })?;

    // Send the request and receive the response
    let client = Client::new();
    let mut forward_req = client.request(parts.method.clone(), uri_str);

    for (name, value) in parts.headers.iter() {
        forward_req = forward_req.header(name, value);
    }

    forward_req = forward_req.body(body_bytes);

    let resp = forward_req
        .send()
        .await
        .map_err(|_| StatusCode::BAD_GATEWAY.with_debug("Could not forward request"))?;

    let mut response_builder = Response::builder().status(resp.status());

    for (key, value) in resp.headers().iter() {
        response_builder = response_builder.header(key, value);
    }

    let resp_bytes = resp.bytes().await.map_err(|_| {
        StatusCode::BAD_GATEWAY.with_debug("Invalid response body. Could not convert to bytes")
    })?;

    let response = response_builder
        .body(Body::from(resp_bytes))
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR.with_debug("Could not build response"))?;

    Ok(response)
}

pub(crate) async fn ws_handler(
    Extension(ParsedURI { prefix: _, subpath }): Extension<ParsedURI>,
    Extension(_): Extension<Service>,
    Extension(Instance(uri)): Extension<Instance>,
    ws: WebSocketUpgrade,
) -> Result<Response<Body>, StatusCode> {
    let uri_str = format!("ws://{uri}{subpath}");
    let uri: Uri = uri_str
        .parse()
        .map_err(|_| StatusCode::BAD_GATEWAY.with_debug("Invalid uri. Could not parse"))?;

    Ok(ws.on_upgrade(move |ws| proxy_websockets(ws, uri)))
}

async fn proxy_websockets(mut client_socket: WebSocket, uri: Uri) {
    let url = uri.to_string();

    match connect_async(&url).await {
        Ok((target_socket, _)) => {
            let (mut client_tx, mut client_rx) = client_socket.split();
            let (mut target_tx, mut target_rx) = target_socket.split();

            let client_to_target = async {
                while let Some(Ok(msg)) = client_rx.next().await {
                    let t_msg = to_tungstenite(msg);
                    if let Err(e) = target_tx.send(t_msg).await {
                        tracing::error!("Error sending to target: {}", e);
                        break;
                    }
                }
                let _ = target_tx.close().await;
            };

            let target_to_client = async {
                while let Some(Ok(msg)) = target_rx.next().await {
                    let c_msg = from_tungstenite(msg);
                    if let Err(e) = client_tx.send(c_msg).await {
                        tracing::error!("Error sending to client: {}", e);
                        break;
                    }
                }
                let _ = client_tx.close().await;
            };

            tokio::select! {
                _ = client_to_target => (),
                _ = target_to_client => (),
            }

            tracing::info!("WebSocket proxy connection closed: {}", url);
        }
        Err(e) => {
            tracing::error!("Failed to connect to upstream WebSocket: {}", e);
            let _ = client_socket
                .send(axum::extract::ws::Message::Close(Some(
                    axum::extract::ws::CloseFrame {
                        code: 1011,
                        reason: axum::extract::ws::Utf8Bytes::from_static(
                            "Upstream connection failed",
                        ),
                    },
                )))
                .await;
        }
    }
}

fn to_tungstenite(msg: Message) -> tokio_tungstenite::tungstenite::Message {
    match msg {
        Message::Text(text) => tokio_tungstenite::tungstenite::Message::Text(
            tokio_tungstenite::tungstenite::Utf8Bytes::from_static(Box::leak(
                text.to_string().into_boxed_str(),
            )),
        ),
        Message::Binary(bin) => tokio_tungstenite::tungstenite::Message::Binary(bin),
        Message::Ping(p) => tokio_tungstenite::tungstenite::Message::Ping(p),
        Message::Pong(p) => tokio_tungstenite::tungstenite::Message::Pong(p),
        Message::Close(Some(CloseFrame { code, reason })) => {
            tokio_tungstenite::tungstenite::Message::Close(Some(
                tokio_tungstenite::tungstenite::protocol::CloseFrame {
                    code: code.into(),
                    reason: tokio_tungstenite::tungstenite::Utf8Bytes::from_static(Box::leak(
                        reason.to_string().into_boxed_str(),
                    )),
                },
            ))
        }
        Message::Close(None) => tokio_tungstenite::tungstenite::Message::Close(None),
    }
}

fn from_tungstenite(msg: tokio_tungstenite::tungstenite::Message) -> Message {
    match msg {
        tokio_tungstenite::tungstenite::Message::Text(text) => Message::Text(
            Utf8Bytes::from_static(Box::leak(text.to_string().into_boxed_str())),
        ),
        tokio_tungstenite::tungstenite::Message::Binary(bin) => Message::Binary(bin),
        tokio_tungstenite::tungstenite::Message::Ping(p) => Message::Ping(p),
        tokio_tungstenite::tungstenite::Message::Pong(p) => Message::Pong(p),
        tokio_tungstenite::tungstenite::Message::Close(Some(
            tokio_tungstenite::tungstenite::protocol::CloseFrame { code, reason },
        )) => Message::Close(Some(CloseFrame {
            code: code.into(),
            reason: Utf8Bytes::from_static(Box::leak(reason.to_string().into_boxed_str())),
        })),
        tokio_tungstenite::tungstenite::Message::Close(None) => Message::Close(None),
        _ => Message::Close(None),
    }
}
