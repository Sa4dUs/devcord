use axum::body::to_bytes;
use axum::{
    Extension,
    body::Body,
    extract::Request,
    http::{StatusCode, Uri},
    response::Response,
};
use hyper::header::CONTENT_LENGTH;
use rand::seq::IndexedRandom;

use reqwest::Client;

use crate::config::{Instance, Service};
use crate::error::ErrorResponse;
use crate::middleware::parser::ParsedURI;

pub(crate) async fn handler(
    Extension(ParsedURI { prefix: _, subpath }): Extension<ParsedURI>,
    Extension(service): Extension<Service>,
    Extension(Instance(uri)): Extension<Instance>,
    mut req: Request,
) -> Result<Response<Body>, StatusCode> {
    let uri_str = format!("http://{uri}{subpath}");
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
