use axum::{
    Router, middleware,
    routing::{delete, get, post, put},
};

use crate::{
    handlers::group_handler::{
        add_users_to_group, create_group, delete_group, list_user_groups, remove_user_from_group,
    },
    state::AppState,
};
use crate::{
    handlers::group_handler::{get_group_id_by_channel, get_group_members},
    middleware::auth::require_auth,
};

pub fn group_routes(state: AppState) -> Router {
    Router::new()
        .route("/create", post(create_group))
        .route("/user-groups", get(list_user_groups))
        .route("/by_channel/{channel_id}", get(get_group_id_by_channel))
        .route("/{group_id}/add-users", put(add_users_to_group))
        .route("/{group_id}/remove-user", post(remove_user_from_group))
        .route("/{group_id}/members", get(get_group_members))
        .route("/{group_id}", delete(delete_group))
        .layer(axum::middleware::from_fn(require_auth))
        .with_state(state)
}
