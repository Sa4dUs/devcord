pub mod status_code;

pub trait ErrorResponse {
    fn with_debug(self, message: &str) -> Self;
}
