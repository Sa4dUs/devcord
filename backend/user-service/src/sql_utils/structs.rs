use std::fmt::Display;

use sqlx::prelude::FromRow;

use crate::api_utils::types::UserID;

#[allow(dead_code)]
#[derive(FromRow, Debug, Default)]
pub struct User {
    pub id: UserID,
    pub username: String,
    pub created_at: Option<String>,
}

#[allow(dead_code)]
#[derive(Debug, Default)]
pub enum FriendRequestState {
    #[default]
    Pending,
    Accepted,
    Denied,
}

impl From<&str> for FriendRequestState {
    fn from(value: &str) -> Self {
        match value.to_lowercase().as_str() {
            "pending" => Self::Pending,
            "accepted" => Self::Accepted,
            "denied" => Self::Denied,
            _ => Self::default(),
        }
    }
}

impl Display for FriendRequestState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FriendRequestState::Pending => write!(f, "pending"),
            FriendRequestState::Accepted => write!(f, "accepted"),
            FriendRequestState::Denied => write!(f, "denied"),
        }
    }
}

#[allow(dead_code)]
#[derive(FromRow, Debug, Default)]
pub struct FriendRequest {
    pub from_user_id: UserID,
    pub to_user_id: UserID,
    pub status: String,
    pub created_at: Option<String>,
}
