use std::sync::Arc;


use axum::{http::Request, response::IntoResponse};

use crate::app::AppState;

pub async fn update<T>(req: Request<T>) -> impl IntoResponse {
    
}