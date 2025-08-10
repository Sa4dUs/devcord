use sqlx::FromRow;

// Just the min to verify the password (faster and it doesn't expose the other data)
#[derive(Clone, FromRow)]
pub struct AuthInfo {
    pub id: String,
    pub username: String,
    pub hashed_password: String,
}
