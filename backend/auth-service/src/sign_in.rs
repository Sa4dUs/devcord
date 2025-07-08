use axum::extract::State;
use axum::{Json, http::StatusCode, response::IntoResponse};
use bincode;
use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};

use crate::api_utils::responses::INTERNAL_SERVER_ERROR;
use crate::db::operations::verify_user_credentials;
use crate::jwt::generate_jwt;
use crate::models::app_state::AppState;
use topic_structs::UserLoggedIn;

#[derive(Serialize)]
struct SignInResponse {
    token: String,
    user_id: String,
    username: String,
}

#[derive(Deserialize)]
pub struct SignInData {
    username: String,
    password: String,
}

pub async fn sign_in_user(
    State(state): State<AppState>,
    Json(entering_user): Json<SignInData>,
) -> impl IntoResponse {
    let auth_info =
        match verify_user_credentials(&state.db, &entering_user.username, &entering_user.password)
            .await
        {
            Some(info) => info,
            None => return (StatusCode::UNAUTHORIZED, "Invalid credentials").into_response(),
        };

    let login_time = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs() as i64;

    let event = UserLoggedIn {
        id: auth_info.clone().id,
        username: auth_info.username.clone(),
        login_time,
    };

    let payload = match bincode::encode_to_vec(&event, bincode::config::standard()) {
        Ok(bytes) => bytes,
        Err(_) => return INTERNAL_SERVER_ERROR.into_response(),
    };

    if let Err(e) = state.producer.send(fluvio::RecordKey::NULL, payload).await {
        tracing::error!("The event logging couldn't be sent through Fluvio: {:?}", e);
    }

    let token = match generate_jwt(auth_info.clone().id) {
        Ok(t) => t,
        Err(_) => return INTERNAL_SERVER_ERROR.into_response(),
    };

    let response = SignInResponse {
        token,
        user_id: auth_info.id,
        username: auth_info.username,
    };

    Json(response).into_response()
}
