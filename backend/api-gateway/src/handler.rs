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
use crate::middleware::parser::ParsedURI;

pub(crate) async fn handler(
    Extension(ParsedURI { prefix: _, subpath }): Extension<ParsedURI>,
    Extension(service): Extension<Service>,
    mut req: Request,
) -> Result<Response<Body>, StatusCode> {
    // MIDDLEWARE2: Check Auth
    // TODO(Sa4dUs): Check JWT if `route.protected`

    // MIDDLEWARE3: Load balancer
    // TODO(Sa4dUs): Move load balancer logic away from here
    let Instance(uri) = service
        .instances
        .choose(&mut rand::rng())
        .ok_or(StatusCode::BAD_GATEWAY)?;

    // MAIN
    // MIDDLEWARE Body to bytes?
    let uri_str = format!("http://{uri}{subpath}");
    let uri: Uri = uri_str.parse().map_err(|_| StatusCode::BAD_GATEWAY)?;
    *req.uri_mut() = uri;

    let (parts, body) = req.into_parts();

    let content_length = parts
        .headers
        .get(CONTENT_LENGTH)
        .and_then(|val| val.to_str().ok()?.parse::<usize>().ok())
        .unwrap_or(0);

    let body_bytes = to_bytes(body, content_length)
        .await
        .map_err(|_| StatusCode::BAD_GATEWAY)?;

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
        .map_err(|_| StatusCode::BAD_GATEWAY)?;

    let mut response_builder = Response::builder().status(resp.status());

    for (key, value) in resp.headers().iter() {
        response_builder = response_builder.header(key, value);
    }

    let resp_bytes = resp.bytes().await.map_err(|_| StatusCode::BAD_GATEWAY)?;

    let response = response_builder
        .body(Body::from(resp_bytes))
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(response)
}
