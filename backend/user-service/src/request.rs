use std::sync::Arc;

use axum::{Json, extract::State, response::IntoResponse};
use serde::Deserialize;

use crate::{
    api_responses,
    app::AppState,
    sql_utils::{self, is_user_in_db},
};

#[derive(Deserialize)]
pub struct Request {
    pub requester: String,
    pub target: String,
}

pub async fn request(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<Request>,
) -> impl IntoResponse {
    if !is_user_in_db(&payload.requester, &state.db).await
        || !is_user_in_db(&payload.target, &state.db).await
    {
        return api_responses::USER_DOES_NOT_EXIST;
    }

    if let Err(_) = sql_utils::create_request(&payload.requester, &payload.target, &state.db).await
    {
        return api_responses::DB_ERROR;
    }

    api_responses::REQUEST_CREATED
}
