use crate::db::operations::{UserInsertError, insert_user};
use crate::jwt::generate_jwt;
use axum::{Extension, Json, http::StatusCode, response::IntoResponse};
use serde::Deserialize;
use serde_json::json;
use sqlx::PgPool;
use std::sync::Arc;

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
            Ok(token) => Json(json!({
                "token": token,
                "user_id": user_info.id,
                "username": user_info.username
            }))
            .into_response(),

            Err(_) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "No se pudo generar el token",
            )
                .into_response(),
        },

        Err(UserInsertError::UsernameTaken) => {
            (StatusCode::CONFLICT, "Nombre de usuario ya en uso").into_response()
        }

        Err(UserInsertError::Database(_)) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            "Error interno del servidor",
        )
            .into_response(),
    }
}
