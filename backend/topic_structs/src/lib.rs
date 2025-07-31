use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct UserCreated {
    pub id: String,
    pub username: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct UserUpdated {
    pub id: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct UserLoggedIn {
    pub id: String,
    pub username: String,
    pub login_time: i64,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct UserLoggedOut {
    pub id: String,
    pub logout_time: i64,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct FriendRequestCreated {
    pub from_username: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct FriendRequestAnswered {
    pub from_username: String,
    pub accepted: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct MessageSent {
    pub channel_id: String,
    pub sender: String,
    pub message: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct GroupCreatedEvent {
    pub group_id: String,
    pub owner_id: String,
    pub channel_id: String,
    pub member_ids: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct GroupDeletedEvent {
    pub group_id: String,
    pub owner_id: String,
    pub member_ids: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct GroupUserAddedEvent {
    pub group_id: String,
    pub user_id: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct GroupUserRemovedEvent {
    pub group_id: String,
    pub user_id: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "type")]
pub enum GroupEvent {
    GroupCreatedEvent(GroupCreatedEvent),
    GroupDeletedEvent(GroupDeletedEvent),
    GroupUserAddedEvent(GroupUserAddedEvent),
    GroupUserRemovedEvent(GroupUserRemovedEvent),
}
