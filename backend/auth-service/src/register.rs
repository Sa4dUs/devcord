use crate::api_utils::responses::{INTERNAL_SERVER_ERROR, USERNAME_ALREADY_USED};
use crate::db::operations::{UserInsertError, insert_user};
use crate::jwt::generate_jwt;
use axum::{Extension, Json, http::StatusCode, response::IntoResponse};
use serde::{Deserialize, Serialize};
use serde_json::json;
use sqlx::PgPool;
use std::sync::Arc;

// Probably will be changed
#[derive(Serialize)]
struct RegisterResponse {
    token: String,
    user_id: i32,
    username: String,
}

#[derive(Deserialize)]
pub struct RegisterData {
    username: String,
    password: String,
    telephone: Option<String>,
}

pub async fn register_user(
    Extension(pool): Extension<Arc<PgPool>>,
    Json(entering_user): Json<RegisterData>,
) -> impl IntoResponse {
    match insert_user(
        &pool,
        &entering_user.username,
        &entering_user.password,
        entering_user.telephone.as_deref(),
    )
    .await
    {
        Ok(user_info) => match generate_jwt(user_info.id) {
            Ok(token) => {
                let response = RegisterResponse {
                    token,
                    user_id: user_info.id,
                    username: user_info.username,
                };
                Json(response).into_response()
            }

            Err(_) => INTERNAL_SERVER_ERROR.into_response(),
        },

        Err(UserInsertError::UsernameTaken) => USERNAME_ALREADY_USED.into_response(),

        Err(UserInsertError::Database(_)) => INTERNAL_SERVER_ERROR.into_response(),
    }
}
