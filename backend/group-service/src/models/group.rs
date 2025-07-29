use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use uuid::Uuid;

#[derive(Deserialize)]
pub struct CreateGroupRequest {
    pub member_ids: Vec<Uuid>,
}

#[derive(Deserialize)]
pub struct AddUsersRequest {
    pub user_ids: Vec<Uuid>,
}

#[derive(Deserialize)]
pub struct RemoveUserRequest {
    pub user_id: Uuid,
}

#[derive(Serialize, FromRow)]
pub struct GroupInfo {
    pub id: Uuid,
    pub owner_id: Uuid,
    pub channel_id: Uuid,
    pub created_at: chrono::NaiveDateTime,
}
