use sqlx::FromRow;

#[derive(FromRow)]
pub struct UserInfo {
    pub id: i32,
    pub username: String,
    pub hashed_password: String,
    pub email: String,
    pub telephone: Option<String>,
}
