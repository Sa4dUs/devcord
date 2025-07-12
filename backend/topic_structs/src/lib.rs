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
