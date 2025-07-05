use axum::extract::State;
use axum::{Json, http::StatusCode, response::IntoResponse};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

use crate::api_utils::responses::INTERNAL_SERVER_ERROR;
use crate::db::operations::verify_user_credentials;
use crate::jwt::generate_jwt;

#[derive(Serialize)]
struct SignInResponse {
    token: String,
    user_id: i32,
    username: String,
}

#[derive(Deserialize)]
pub struct SignInData {
    username: String,
    password: String,
}

pub async fn sign_in_user(
    State(pool): State<PgPool>,
    Json(entering_user): Json<SignInData>,
) -> impl IntoResponse {
    if let Some(auth_info) =
        verify_user_credentials(&pool, &entering_user.username, &entering_user.password).await
    {
        match generate_jwt(auth_info.id) {
            Ok(token) => {
                let response = SignInResponse {
                    token,
                    user_id: auth_info.id,
                    username: auth_info.username,
                };
                Json(response).into_response()
            }
            Err(_) => INTERNAL_SERVER_ERROR.into_response(),
        }
    } else {
        (StatusCode::UNAUTHORIZED, "Invalid credentials").into_response()
    }
}
