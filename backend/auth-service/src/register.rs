use axum::{Json, response::IntoResponse};
use serde::Deserialize;

struct RegisterData {
    username: String,
    password: String,
    telephone: Option<String>,
}

pub async func register_user(Json(entering_user) : Json<RegisterData>) -> impl IntoResponse{
    
}