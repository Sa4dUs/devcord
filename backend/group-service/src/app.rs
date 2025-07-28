use axum::Router;
use sqlx::PgPool;

use crate::{middleware::auth::require_auth, routes::group_routes};

pub fn build(db: PgPool) -> Router {
    let protected = Router::new()
        .merge(group_routes(db))
        .layer(axum::middleware::from_fn(require_auth));
    Router::new().merge(protected)
}
