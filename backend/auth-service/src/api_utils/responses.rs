use axum::{Json, http::StatusCode};
use serde::Serialize;

#[derive(Serialize, Clone, Copy)]
pub struct ApiResponseMessage {
    pub message: &'static str,
}

pub type ApiResponse = (StatusCode, Json<ApiResponseMessage>);

pub static USERNAME_ALREADY_USED: ApiResponse = (
    StatusCode::BAD_REQUEST,
    Json(ApiResponseMessage {
        message: "Username is already used",
    }),
);

pub static INTERNAL_SERVER_ERROR: ApiResponse = (
    StatusCode::INTERNAL_SERVER_ERROR,
    Json(ApiResponseMessage {
        message: "Internal server error",
    }),
);
