use std::sync::Arc;

use axum::{Router, middleware, routing::get};

use crate::middleware::auth::require_auth;
use crate::middleware::channel::extract_channel;
use crate::{handlers::message_handler::message_handler, state::AppState};

pub fn message_routes(state: AppState) -> Router {
    Router::new()
        .route("/", get(message_handler))
        .layer(middleware::from_fn(extract_channel))
        .layer(middleware::from_fn(require_auth))
        .with_state(Arc::new(state))
}
