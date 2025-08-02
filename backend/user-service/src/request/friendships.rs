use std::sync::Arc;

use axum::{
    Json,
    extract::{Query, State},
    response::IntoResponse,
};
use axum_extra::either::Either::{self, E1, E2};
use serde_json::to_vec;
use topic_structs::{FriendRequestAnswered, FriendRequestCreated};

use crate::{
    api_utils::{
        responses,
        structs::{
            FriendRequestState, PublicFriendRequestReceived, PublicFriendRequestSent,
            PublicFriendship, RequestFriendRequest, RequestFriendRequestRecieved,
            RequestFriendRequestSent, RequestFriendships,
        },
    },
    app::AppState,
    jwt::Claims,
    sql_utils::calls::{
        get_private_block, get_private_friend_request, get_private_user,
        get_public_friend_requests_received, get_public_friend_requests_sent,
        get_public_friendships, get_public_user, insert_friend_request, insert_friendship,
        update_friend_request_state,
    },
};

pub async fn request_friend(
    State(state): State<Arc<AppState>>,
    claims: Claims,
    Json(body): Json<RequestFriendRequest>,
) -> impl IntoResponse {
    let Some(from_user) = get_public_user(&claims.user_id, &state.db).await else {
        return responses::USER_DOES_NOT_EXIST;
    };

    let to_user = match get_private_user(&body.to_user_username, &state.db).await {
        Some(e) => e,
        None => return responses::USER_DOES_NOT_EXIST,
    };

    if get_private_block(&to_user.id, &claims.user_id, &state.db)
        .await
        .is_some()
    {
        return responses::USER_DOES_NOT_EXIST;
    }

    if get_private_friend_request(&claims.user_id, &to_user.id, &state.db)
        .await
        .is_some()
    {
        return responses::REQUEST_ALREADY_EXIST;
    }

    if insert_friend_request(&claims.user_id, &to_user.id, &state.db)
        .await
        .is_err()
    {
        return responses::DB_ERROR;
    }

    let request = FriendRequestCreated {
        from_username: from_user.username,
    };

    let Ok(request_bytes) = to_vec(&request) else {
        return responses::FLUVIO_ERROR;
    };

    if state
        .request_sent_producer
        .send(to_user.id, request_bytes)
        .await
        .is_err()
    {
        return responses::FLUVIO_ERROR;
    }

    responses::REQUEST_CREATED
}

pub async fn accept_friend(
    State(state): State<Arc<AppState>>,
    claims: Claims,
    Json(body): Json<RequestFriendRequest>,
) -> impl IntoResponse {
    let Some(_to_user) = get_public_user(&claims.user_id, &state.db).await else {
        return responses::USER_DOES_NOT_EXIST;
    };

    let from_user = match get_private_user(&body.to_user_username, &state.db).await {
        Some(e) => e,
        None => return responses::USER_DOES_NOT_EXIST,
    };

    let mut request =
        match get_private_friend_request(&from_user.id, &claims.user_id, &state.db).await {
            Some(e) => e,
            None => return responses::REQUEST_DOES_NOT_EXIST,
        };

    if request.state != FriendRequestState::Pending.to_string() {
        return responses::REQUEST_NOT_PENDING;
    }

    request.state = FriendRequestState::Accepted.to_string();
    if update_friend_request_state(request, &state.db)
        .await
        .is_err()
    {
        return responses::DB_ERROR;
    }

    if insert_friendship(&from_user.id, &claims.user_id, &state.db)
        .await
        .is_err()
    {
        return responses::DB_ERROR;
    }

    let request = FriendRequestAnswered {
        from_username: from_user.username,
        accepted: true,
    };

    let Ok(request_bytes) = to_vec(&request) else {
        return responses::FLUVIO_ERROR;
    };

    if state
        .request_answered_producer
        .send(from_user.id, request_bytes)
        .await
        .is_err()
    {
        return responses::FLUVIO_ERROR;
    }

    responses::REQUEST_ACCEPTED
}

pub async fn reject_friend(
    State(state): State<Arc<AppState>>,
    claims: Claims,
    Json(body): Json<RequestFriendRequest>,
) -> impl IntoResponse {
    let Some(_to_user) = get_public_user(&claims.user_id, &state.db).await else {
        return responses::USER_DOES_NOT_EXIST;
    };

    let from_user = match get_private_user(&body.to_user_username, &state.db).await {
        Some(e) => e,
        None => return responses::USER_DOES_NOT_EXIST,
    };

    let mut request =
        match get_private_friend_request(&from_user.id, &claims.user_id, &state.db).await {
            Some(e) => e,
            None => return responses::REQUEST_DOES_NOT_EXIST,
        };

    if request.state != FriendRequestState::Pending.to_string() {
        return responses::REQUEST_NOT_PENDING;
    }

    request.state = FriendRequestState::Rejected.to_string();
    if update_friend_request_state(request, &state.db)
        .await
        .is_err()
    {
        return responses::DB_ERROR;
    }

    let request = FriendRequestAnswered {
        from_username: from_user.username,
        accepted: false,
    };

    let Ok(request_bytes) = to_vec(&request) else {
        return responses::FLUVIO_ERROR;
    };

    if state
        .request_answered_producer
        .send(from_user.id, request_bytes)
        .await
        .is_err()
    {
        return responses::FLUVIO_ERROR;
    }

    responses::REQUEST_REJECTED
}

pub async fn get_request_sent(
    State(state): State<Arc<AppState>>,
    claims: Claims,
    Query(query): Query<RequestFriendRequestSent>,
) -> Either<Json<Option<Vec<PublicFriendRequestSent>>>, impl IntoResponse> {
    if get_public_user(&claims.user_id, &state.db).await.is_none() {
        return E2(responses::USER_DOES_NOT_EXIST);
    }

    let requests =
        get_public_friend_requests_sent(&claims.user_id, query.from, query.to, &state.db).await;

    E1(Json(requests))
}

pub async fn get_request_received(
    State(state): State<Arc<AppState>>,
    claims: Claims,
    Query(query): Query<RequestFriendRequestRecieved>,
) -> Either<Json<Option<Vec<PublicFriendRequestReceived>>>, impl IntoResponse> {
    if get_public_user(&claims.user_id, &state.db).await.is_none() {
        return E2(responses::USER_DOES_NOT_EXIST);
    }

    let requests =
        get_public_friend_requests_received(&claims.user_id, query.from, query.to, &state.db).await;

    E1(Json(requests))
}

pub async fn get_friends(
    State(state): State<Arc<AppState>>,
    claims: Claims,
    Query(query): Query<RequestFriendships>,
) -> Either<Json<Option<Vec<PublicFriendship>>>, impl IntoResponse> {
    if get_public_user(&claims.user_id, &state.db).await.is_none() {
        return E2(responses::USER_DOES_NOT_EXIST);
    }

    let requests = get_public_friendships(&claims.user_id, query.from, query.to, &state.db).await;

    E1(Json(requests))
}
