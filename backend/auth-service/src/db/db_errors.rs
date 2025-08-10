use sqlx::Error as SqlxError;
use thiserror::Error;

const UNIQUE_VIOLATED: &str = "23505";

#[derive(Debug, Error)]
pub enum UserInsertError {
    #[error("The email or the username is already in used")]
    UsernameTaken,
    #[error("Database not working properly")]
    Database(SqlxError),
}

impl From<SqlxError> for UserInsertError {
    fn from(err: SqlxError) -> Self {
        if let SqlxError::Database(db_err) = &err {
            if db_err.code().as_deref() == Some(UNIQUE_VIOLATED) {
                return UserInsertError::UsernameTaken;
            }
        }
        UserInsertError::Database(err)
    }
}
