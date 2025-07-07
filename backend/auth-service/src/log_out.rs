use crate::api_utils::responses::INTERNAL_SERVER_ERROR;
use crate::models::app_state::AppState;
use axum::extract::State;
use axum::{Json, response::IntoResponse};
use bincode::{config::standard, encode_to_vec};
use chrono::Utc;
use topic_structs::UserLoggedOut;
use tracing::error;

pub async fn log_user_outf(State(state): State<AppState>, user_id: String) -> impl IntoResponse {
    let logout_time = Utc::now().timestamp();

    let event = UserLoggedOut {
        id: user_id.clone(),
        logout_time,
    };

    let event_bytes = match encode_to_vec(event, standard()) {
        Ok(bytes) => bytes,
        Err(_) => return INTERNAL_SERVER_ERROR.into_response(),
    };

    if let Err(e) = state.producer.send(&*user_id, event_bytes).await {
        error!("Failed to send UserLoggedOut event to Fluvio: {}", e);
        return INTERNAL_SERVER_ERROR.into_response();
    }

    Json("User logged out successfully").into_response()
}
