pub mod status_code;

use axum::response::Response;

pub trait ErrorResponse {
    fn with_debug(self, message: &str) -> Self;
}
