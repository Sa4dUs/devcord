use std::sync::Arc;

use axum::{
    Json,
    extract::{Query, State},
    response::IntoResponse,
};
use axum_extra::either::Either::{self, E1, E2};

use crate::{
    api_utils::{
        responses,
        structs::{PrivateBlocked, PublicBlocked, RequestUserBlock, RequestUsersBlocked},
    },
    app::AppState,
    jwt::Claims,
    sql_utils::calls::{
        delete_block, delete_friend_request, delete_friendship, get_private_block,
        get_private_friend_request, get_private_friendship, get_private_user, get_public_blocks,
        get_public_user, get_undirected_private_friend_requests, insert_block,
    },
};

pub async fn block_user(
    State(state): State<Arc<AppState>>,
    claims: Claims,
    Json(body): Json<RequestUserBlock>,
) -> impl IntoResponse {
    if get_public_user(&claims.user_id, &state.db).await.is_none() {
        return responses::USER_DOES_NOT_EXIST;
    }

    let to_user = match get_private_user(&body.to_user_username, &state.db).await {
        Some(e) => e,
        None => return responses::USER_DOES_NOT_EXIST,
    };

    if get_private_block(&claims.user_id, &to_user.id, &state.db)
        .await
        .is_some()
    {
        return responses::BLOCK_ALREADY_EXISTS;
    }

    let block = PrivateBlocked {
        from_user_id: claims.user_id.clone(),
        to_user_id: to_user.id.clone(),
        created_at: None,
    };

    if insert_block(block, &state.db).await.is_err() {
        return responses::DB_ERROR;
    }

    // Prob should make this better but im too lazy rn
    if let Some(mut requests) =
        get_undirected_private_friend_requests(&claims.user_id, &to_user.id, &state.db).await
    {
        if let Some(request) = requests.pop() {
            if delete_friend_request(request, &state.db).await.is_err() {
                return responses::DB_ERROR;
            }
        }
    };

    if let Some(friendship) = get_private_friendship(&claims.user_id, &to_user.id, &state.db).await
    {
        if delete_friendship(friendship, &state.db).await.is_err() {
            return responses::DB_ERROR;
        }
    };

    responses::BLOCK_ADDED
}

pub async fn unblock_user(
    State(state): State<Arc<AppState>>,
    claims: Claims,
    Json(body): Json<RequestUserBlock>,
) -> impl IntoResponse {
    if get_public_user(&claims.user_id, &state.db).await.is_none() {
        return responses::USER_DOES_NOT_EXIST;
    }

    let to_user = match get_private_user(&body.to_user_username, &state.db).await {
        Some(e) => e,
        None => return responses::USER_DOES_NOT_EXIST,
    };

    let block = match get_private_block(&claims.user_id, &to_user.id, &state.db).await {
        Some(e) => e,
        None => return responses::BLOCK_DOES_NOT_EXISTS,
    };

    if delete_block(block, &state.db).await.is_err() {
        return responses::DB_ERROR;
    }

    responses::BLOCK_REMOVED
}

pub async fn get_blocked(
    State(state): State<Arc<AppState>>,
    claims: Claims,
    Query(query): Query<RequestUsersBlocked>,
) -> Either<Json<Option<Vec<PublicBlocked>>>, impl IntoResponse> {
    if get_public_user(&claims.user_id, &state.db).await.is_none() {
        return E2(responses::USER_DOES_NOT_EXIST);
    }

    let requests = get_public_blocks(&claims.user_id, query.from, query.to, &state.db).await;

    E1(Json(requests))
}
