use std::sync::Arc;

use axum::{Json, extract::State};
use serde::Deserialize;

use crate::{
    api_utils::responses::{self, ApiResponse},
    app::AppState,
    sql_utils::{
        calls::{self, create_friendship, get_friend_request, update_friend_request_state},
        structs::FriendRequestState::{self, Accepted, Denied, Pending},
    },
};

#[derive(Deserialize)]
pub struct Request {
    pub requester: String,
    pub target: String,
}

pub async fn request(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<Request>,
) -> ApiResponse {
    if let Err(e) = both_users_exist(&payload.requester, &payload.target, &state.db).await {
        return e;
    }

    if calls::create_friend_request(&payload.requester, &payload.target, &state.db)
        .await
        .is_err()
    {
        return responses::DB_ERROR;
    }

    responses::REQUEST_CREATED
}

pub async fn accept(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<Request>,
) -> ApiResponse {
    if let Err(e) = both_users_exist(&payload.requester, &payload.target, &state.db).await {
        return e;
    }

    let request = get_friend_request(&payload.requester, &payload.target, &state.db).await;

    match request {
        Err(_) => return responses::REQUEST_DOES_NOT_EXIST,
        Ok(req) => match FriendRequestState::from(req.status.as_str()) {
            Pending => {
                if update_friend_request_state(
                    &payload.requester,
                    &payload.target,
                    &state.db,
                    Accepted,
                )
                .await
                .is_err()
                {
                    return responses::DB_ERROR;
                }

                if create_friendship(&payload.requester, &payload.target, &state.db)
                    .await
                    .is_err()
                {
                    return responses::DB_ERROR;
                }
            }
            _ => return responses::REQUEST_NOT_PENDING,
        },
    }

    responses::REQUEST_DENIED
}

pub async fn deny(State(state): State<Arc<AppState>>, Json(payload): Json<Request>) -> ApiResponse {
    if let Err(e) = both_users_exist(&payload.requester, &payload.target, &state.db).await {
        return e;
    }

    let request = get_friend_request(&payload.requester, &payload.target, &state.db).await;

    match request {
        Err(_) => return responses::REQUEST_DOES_NOT_EXIST,
        Ok(req) => match FriendRequestState::from(req.status.as_str()) {
            Pending => {
                if update_friend_request_state(
                    &payload.requester,
                    &payload.target,
                    &state.db,
                    Denied,
                )
                .await
                .is_err()
                {
                    return responses::DB_ERROR;
                }

                if create_friendship(&payload.requester, &payload.target, &state.db)
                    .await
                    .is_err()
                {
                    return responses::DB_ERROR;
                }
            }
            _ => return responses::REQUEST_NOT_PENDING,
        },
    }

    responses::REQUEST_ACCEPTED
}

async fn both_users_exist(
    requester: &str,
    target: &str,
    db: &sqlx::PgPool,
) -> Result<(), ApiResponse> {
    if calls::get_user(requester, db).await.is_err() || calls::get_user(target, db).await.is_err() {
        return Err(responses::USER_DOES_NOT_EXIST);
    }

    Ok(())
}
