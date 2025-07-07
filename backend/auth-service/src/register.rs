use crate::api_utils::responses::{INTERNAL_SERVER_ERROR, USERNAME_ALREADY_USED};
use crate::db::operations::{UserInsertError, insert_user};
use crate::db::password_hasher::hash_password;
use crate::jwt::generate_jwt;
use crate::models::app_state::AppState;

use axum::extract::State;
use axum::{Json, response::IntoResponse};
use bincode::{Encode, config::standard, encode_to_vec};
use fluvio::TopicProducer;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use topic_structs::UserCreated;

#[derive(Serialize)]
struct RegisterResponse {
    token: String,
    user_id: String,
    username: String,
}

#[derive(Deserialize)]
pub struct RegisterData {
    username: String,
    password: String,
    email: String,
    telephone: Option<String>,
}

pub async fn register_user(
    State(state): State<AppState>,
    Json(entering_user): Json<RegisterData>,
) -> impl IntoResponse {
    let hashed_password = match hash_password(&entering_user.password).await {
        Ok(p) => p,
        Err(_) => return INTERNAL_SERVER_ERROR.into_response(),
    };

    let user_info = match insert_user(
        &state.db,
        &entering_user.username,
        &hashed_password,
        &entering_user.email,
        entering_user.telephone.as_deref(),
    )
    .await
    {
        Ok(user) => user,
        Err(UserInsertError::UsernameTaken) => return USERNAME_ALREADY_USED.into_response(),
        Err(UserInsertError::Database(_)) => return INTERNAL_SERVER_ERROR.into_response(),
    };

    let token = match generate_jwt(&user_info.id) {
        Ok(t) => t,
        Err(_) => return INTERNAL_SERVER_ERROR.into_response(),
    };

    let event = UserCreated {
        id: user_info.id,
        username: user_info.username.clone(),
    };

    let event_bytes = match encode_to_vec(event, standard()) {
        Ok(bytes) => bytes,
        Err(_) => return INTERNAL_SERVER_ERROR.into_response(),
    };

    if let Err(e) = state
        .producer
        .send(&user_info.id.to_string(), event_bytes)
        .await
    {
        eprintln!("Failed to send event to Fluvio: {}", e);
        return INTERNAL_SERVER_ERROR.into_response();
    }

    let response = RegisterResponse {
        token,
        user_id: user_info.id,
        username: user_info.username,
    };

    Json(response).into_response()
}
