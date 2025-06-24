use bincode::{Decode, Encode};

#[allow(dead_code)]
#[derive(Encode, Decode, Debug, Clone, Default)]
pub struct UserCreated {
    pub id: String,
    pub username: String,
}

#[allow(dead_code)]
#[derive(Encode, Decode, Debug, Clone, Default)]
pub struct UserUpdated {
    pub id: String,
}
