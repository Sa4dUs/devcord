use serde::Serialize;
use sqlx::prelude::FromRow;
use uuid::Uuid;

#[derive(Serialize, FromRow)]
pub struct MessageInfo {
    pub id: Uuid,
    pub sender_id: Uuid,
    pub channel_id: Uuid,
    pub message: String,
    pub created_at: chrono::NaiveDateTime,
}
