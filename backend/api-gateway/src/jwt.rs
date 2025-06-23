use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct Claims {
    // TODO(Sa4dUs): Add more claim fields if needed
    exp: u64,
    // Actual payload
    user_id: String,
}
