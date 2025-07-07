use axum::{Json, http::StatusCode};
use serde::Serialize;
#[derive(Serialize, Clone, Copy)]
pub struct ApiResponseMessage {
    pub message: &'static str,
}

pub type ApiResponse<T> = (StatusCode, Json<T>);

pub static USER_DOES_NOT_EXIST: ApiResponse<ApiResponseMessage> = (
    StatusCode::NOT_FOUND,
    Json(ApiResponseMessage {
        message: "User does no exist",
    }),
);

pub static ILLEGAL_QUERY: ApiResponse<ApiResponseMessage> = (
    StatusCode::BAD_REQUEST,
    Json(ApiResponseMessage {
        message: "Illegal query",
    }),
);

pub static FLUVIO_ERROR: ApiResponse<ApiResponseMessage> = (
    StatusCode::INTERNAL_SERVER_ERROR,
    Json(ApiResponseMessage {
        message: "Failed to communicate with fluvio",
    }),
);

pub static DB_ERROR: ApiResponse<ApiResponseMessage> = (
    StatusCode::INTERNAL_SERVER_ERROR,
    Json(ApiResponseMessage {
        message: "Failed to save to db",
    }),
);

pub static USER_UPDATED: ApiResponse<ApiResponseMessage> = (
    StatusCode::OK,
    Json(ApiResponseMessage {
        message: "User updated",
    }),
);

pub static REQUEST_CREATED: ApiResponse<ApiResponseMessage> = (
    StatusCode::OK,
    Json(ApiResponseMessage {
        message: "Request created",
    }),
);

pub static REQUEST_ACCEPTED: ApiResponse<ApiResponseMessage> = (
    StatusCode::OK,
    Json(ApiResponseMessage {
        message: "Request accepted",
    }),
);

pub static REQUEST_REJECTED: ApiResponse<ApiResponseMessage> = (
    StatusCode::OK,
    Json(ApiResponseMessage {
        message: "Request rejected",
    }),
);

pub static REQUEST_DOES_NOT_EXIST: ApiResponse<ApiResponseMessage> = (
    StatusCode::NOT_FOUND,
    Json(ApiResponseMessage {
        message: "Request does not exist",
    }),
);

pub static REQUEST_ALREADY_EXIST: ApiResponse<ApiResponseMessage> = (
    StatusCode::CONFLICT,
    Json(ApiResponseMessage {
        message: "Request already exist",
    }),
);

pub static REQUEST_NOT_PENDING: ApiResponse<ApiResponseMessage> = (
    StatusCode::CONFLICT,
    Json(ApiResponseMessage {
        message: "Friend request already answered",
    }),
);

pub static BLOCK_ADDED: ApiResponse<ApiResponseMessage> = (
    StatusCode::OK,
    Json(ApiResponseMessage {
        message: "Block added",
    }),
);

pub static BLOCK_REMOVED: ApiResponse<ApiResponseMessage> = (
    StatusCode::OK,
    Json(ApiResponseMessage {
        message: "Block removed",
    }),
);

pub static BLOCK_ALREADY_EXISTS: ApiResponse<ApiResponseMessage> = (
    StatusCode::CONFLICT,
    Json(ApiResponseMessage {
        message: "Block already exists",
    }),
);

pub static BLOCK_DOES_NOT_EXISTS: ApiResponse<ApiResponseMessage> = (
    StatusCode::NOT_FOUND,
    Json(ApiResponseMessage {
        message: "Block does not exist",
    }),
);
