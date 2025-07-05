use sqlx::FromRow;

// Just the min to verify the password (faster and it doesn't expose the other data)
#[derive(FromRow)]
pub struct AuthInfo {
    pub id: i32,
    pub username: String,
    pub hashed_password: String,
}
