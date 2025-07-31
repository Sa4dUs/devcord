use axum::{
    Router, middleware,
    routing::{delete, get, post, put},
};

use crate::middleware::auth::require_auth;
use crate::{
    handlers::group_handler::{
        add_users_to_group, create_group, delete_group, list_user_groups, remove_user_from_group,
    },
    state::AppState,
};

pub fn group_routes(state: AppState) -> Router {
    Router::new()
        .route("/create", post(create_group))
        .route("/user-groups", get(list_user_groups))
        .route("/{group_id}/add-users", put(add_users_to_group))
        .route("/{group_id}", delete(delete_group))
        .route("/{group_id}/remove-user", post(remove_user_from_group))
        .layer(middleware::from_fn(require_auth))
        .with_state(state)
}
