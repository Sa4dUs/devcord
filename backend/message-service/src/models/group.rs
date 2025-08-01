use serde::Deserialize;
use uuid::Uuid;

#[derive(Deserialize)]
pub struct GroupIdResponse {
    pub group_id: Uuid,
}

#[derive(Deserialize)]
pub struct GroupMembersResponse {
    pub members: Vec<Uuid>,
}
