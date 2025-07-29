use axum::{
    Router, middleware,
    routing::{delete, get, post, put},
};
use sqlx::PgPool;

use crate::handlers::group_handler::{
    add_users_to_group, create_group, delete_group, list_user_groups, remove_user_from_group,
};
use crate::middleware::auth::require_auth;

pub fn group_routes(db: PgPool) -> Router {
    Router::new()
        .route("/create", post(create_group))
        .route("/user-groups", get(list_user_groups))
        .route("/{group_id}/add-users", put(add_users_to_group))
        .route("/{group_id}", delete(delete_group))
        .route("/{group_id}/remove-user", post(remove_user_from_group))
        .layer(middleware::from_fn(require_auth))
        .with_state(db)
}
