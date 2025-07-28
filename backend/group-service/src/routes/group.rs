use axum::{Router, middleware, routing::post};
use sqlx::PgPool;

use crate::{handlers::group_handler::create_group, middleware::auth::require_auth};

pub fn group_routes(db: PgPool) -> Router {
    Router::new()
        .route("/create", post(create_group))
        .layer(middleware::from_fn(require_auth))
        .with_state(db)
}
