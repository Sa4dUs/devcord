use std::{collections::HashMap, fmt::Display};

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;

use crate::api_utils::types::{UserID, UserUsername};

#[allow(dead_code)]
#[derive(Debug, Deserialize, Serialize, PartialEq, Eq, Hash)]
pub enum RequestUpdateProfileEnum {
    Username,
}

#[allow(dead_code)]
#[derive(FromRow, Debug, Default, Deserialize, Serialize)]
pub struct RequestUpdateProfile {
    pub query: HashMap<RequestUpdateProfileEnum, String>,
}

#[allow(dead_code)]
#[derive(FromRow, Debug, Default, Deserialize, Serialize)]
pub struct RequestUserProfile {
    pub user_username: UserUsername,
}

#[allow(dead_code)]
#[derive(FromRow, Debug, Default, Deserialize, Serialize)]
pub struct PrivateUser {
    pub id: UserID,
    pub username: UserUsername,
    pub created_at: Option<DateTime<Utc>>,
}

#[allow(dead_code)]
#[derive(FromRow, Debug, Default, Deserialize, Serialize)]
pub struct PublicUser {
    pub username: UserUsername,
    pub created_at: Option<DateTime<Utc>>,
}

#[allow(dead_code)]
#[derive(Debug, Default, Deserialize, Serialize)]
pub enum FriendRequestState {
    #[default]
    Pending,
    Accepted,
    Rejected,
}

#[allow(dead_code)]
#[derive(FromRow, Debug, Default, Serialize, Deserialize)]
pub struct PrivateFriendRequest {
    pub from_user_id: UserID,
    pub to_user_id: UserID,
    pub state: String,
    pub created_at: Option<DateTime<Utc>>,
}

#[allow(dead_code)]
#[derive(FromRow, Debug, Default, Serialize, Deserialize)]
pub struct PublicFriendRequestSent {
    #[sqlx(rename = "username")]
    pub to_user_username: UserUsername,
    pub status: String,
    pub created_at: Option<DateTime<Utc>>,
}

#[allow(dead_code)]
#[derive(FromRow, Debug, Default, Serialize, Deserialize)]
pub struct PublicFriendRequestReceived {
    #[sqlx(rename = "username")]
    pub from_user_username: UserUsername,
    pub status: String,
    pub created_at: Option<DateTime<Utc>>,
}

#[allow(dead_code)]
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct RequestFriendRequestRecieved {
    pub from: i64,
    pub to: i64,
}

#[allow(dead_code)]
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct RequestFriendRequestSent {
    pub from: i64,
    pub to: i64,
}

#[allow(dead_code)]
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct RequestFriendRequest {
    pub to_user_username: UserUsername,
}

#[allow(dead_code)]
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct RequestUsersBlocked {
    pub from: i64,
    pub to: i64,
}

#[allow(dead_code)]
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct RequestUserBlock {
    pub to_user_username: UserUsername,
}

#[allow(dead_code)]
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct RequestFriendships {
    pub from: i64,
    pub to: i64,
}

#[allow(dead_code)]
#[derive(FromRow, Debug, Default, Serialize, Deserialize)]
pub struct PrivateFriendship {
    pub from_user_id: UserID,
    pub to_user_id: UserID,
    pub created_at: Option<DateTime<Utc>>,
}

#[allow(dead_code)]
#[derive(FromRow, Debug, Default, Serialize, Deserialize)]
pub struct PublicFriendship {
    #[sqlx(rename = "username")]
    pub to_user_username: UserUsername,
    pub created_at: Option<DateTime<Utc>>,
}

#[allow(dead_code)]
#[derive(FromRow, Debug, Default, Deserialize, Serialize)]
pub struct PrivateBlocked {
    pub from_user_id: UserID,
    pub to_user_id: UserID,
    pub created_at: Option<DateTime<Utc>>,
}

#[allow(dead_code)]
#[derive(FromRow, Debug, Default, Deserialize, Serialize)]
pub struct PublicBlocked {
    #[sqlx(rename = "username")]
    pub to_user_username: UserUsername,
    pub created_at: Option<String>,
}

impl From<&str> for FriendRequestState {
    fn from(value: &str) -> Self {
        match value.to_lowercase().as_str() {
            "pending" => Self::Pending,
            "accepted" => Self::Accepted,
            "rejected" => Self::Rejected,
            _ => Self::default(),
        }
    }
}

impl Display for FriendRequestState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FriendRequestState::Pending => write!(f, "pending"),
            FriendRequestState::Accepted => write!(f, "accepted"),
            FriendRequestState::Rejected => write!(f, "rejected"),
        }
    }
}
