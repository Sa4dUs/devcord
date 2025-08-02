use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct Claims {
    exp: u64,
    // Actual payload
    user_id: String,
}
