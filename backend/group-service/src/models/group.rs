use serde::Deserialize;
use uuid::Uuid;

#[derive(Deserialize)]
pub struct CreateGroupRequest {
    pub member_ids: Vec<Uuid>,
}
