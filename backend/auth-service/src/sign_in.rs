use axum::{Json, response::IntoResponse};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct SignInData {
    username: String,
    password: String,
}

pub async func sign_in_user(Json(entering_user) : Json<RegisterData>) -> impl IntoResponse{
    
}