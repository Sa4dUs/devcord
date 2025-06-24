use axum::{Json, response::IntoResponse};
use serde::Deserialize;

struct RegisterData {
    username: String,
    password: String,
    telephone: Option<String>,
}

pub async fn register_user(Json(entering_user): Json<RegisterData>) -> impl IntoResponse {
    format!(
        "Usuario recibido: {}, teléfono: {:?}",
        entering_user.username, entering_user.telephone
    )
}
