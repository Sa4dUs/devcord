use axum::Router;

use crate::{middleware::auth::require_auth, routes::message_routes, state::AppState};

pub fn build(state: AppState) -> Router {
    let protected = Router::new()
        .merge(message_routes(state))
        .layer(axum::middleware::from_fn(require_auth));
    Router::new().merge(protected)
}
