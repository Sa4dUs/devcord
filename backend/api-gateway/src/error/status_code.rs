use hyper::StatusCode;

use crate::error::ErrorResponse;

impl ErrorResponse for StatusCode {
    fn with_debug(self, message: &str) -> Self {
        tracing::debug!(message);
        self
    }
}
