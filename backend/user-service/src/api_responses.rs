use axum::{Json, http::StatusCode};
use serde::Serialize;
#[derive(Serialize, Clone, Copy)]
pub struct ApiResponseMessage {
    pub message: &'static str,
}

type ApiResponse = (StatusCode, Json<ApiResponseMessage>);

pub static USER_DOES_NOT_EXIST: ApiResponse = (
    StatusCode::BAD_REQUEST,
    Json(ApiResponseMessage {
        message: "User does no exist",
    }),
);

pub static ILLEGAL_QUERY: ApiResponse = (
    StatusCode::BAD_REQUEST,
    Json(ApiResponseMessage {
        message: "Illegal query",
    }),
);

pub static FLUVIO_ERROR: ApiResponse = (
    StatusCode::INTERNAL_SERVER_ERROR,
    Json(ApiResponseMessage {
        message: "Failed to communicate with fluvio",
    }),
);

pub static DB_ERROR: ApiResponse = (
    StatusCode::INTERNAL_SERVER_ERROR,
    Json(ApiResponseMessage {
        message: "Failed to save to db",
    }),
);

pub static USER_UPDATED: ApiResponse = (
    StatusCode::OK,
    Json(ApiResponseMessage {
        message: "User updated",
    }),
);

pub static REQUEST_CREATED: ApiResponse = (
    StatusCode::OK,
    Json(ApiResponseMessage {
        message: "Request created",
    }),
);
