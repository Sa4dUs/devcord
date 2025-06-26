use crate::sql_utils::structs::{FriendRequest, FriendRequestState, User};

//--------------------GETTERS--------------------

pub async fn get_user(id: &str, db: &sqlx::PgPool) -> sqlx::Result<User> {
    sqlx::query_as("SELECT id FROM users WHERE id = $1")
        .bind(id)
        .fetch_one(db)
        .await
}

pub async fn get_friend_request(
    from: &str,
    to: &str,
    db: &sqlx::PgPool,
) -> sqlx::Result<FriendRequest> {
    sqlx::query_as(
        "
        SELECT id FROM friend_requests WHERE from_user_id = $1 AND to_user_id = $2
    ",
    )
    .bind(from)
    .bind(to)
    .fetch_one(db)
    .await
}

//--------------------INSERTS--------------------

pub async fn create_friend_request(from: &str, to: &str, db: &sqlx::PgPool) -> anyhow::Result<()> {
    sqlx::query("INSERT INTO friend_requests (from_user_id, to_user_id) VALUES ($1, $2)")
        .bind(from)
        .bind(to)
        .execute(db)
        .await?;

    Ok(())
}

pub async fn create_friendship(from: &str, to: &str, db: &sqlx::PgPool) -> anyhow::Result<()> {
    sqlx::query("INSERT INTO friendships (from_user_id, to_user_id) VALUES ($1, $2)")
        .bind(from)
        .bind(to)
        .execute(db)
        .await?;

    sqlx::query("INSERT INTO friendships (from_user_id, to_user_id) VALUES ($1, $2)")
        .bind(to)
        .bind(from)
        .execute(db)
        .await?;

    Ok(())
}

//--------------------UPDATE--------------------

pub async fn update_user_username(
    id: &str,
    username: &str,
    db: &sqlx::PgPool,
) -> anyhow::Result<()> {
    sqlx::query(
        "
                UPDATE users
                SET username = $2
                WHERE id = $1
            ",
    )
    .bind(id)
    .bind(username)
    .execute(db)
    .await?;

    Ok(())
}

pub async fn update_friend_request_state(
    from: &str,
    to: &str,
    db: &sqlx::PgPool,
    new_state: FriendRequestState,
) -> anyhow::Result<()> {
    sqlx::query(
        "
        UPDATE friend_requests
        SET state = $1
        WHERE from_user_id = $2 AND to_user_id = $3
        ",
    )
    .bind(new_state.to_string())
    .bind(from)
    .bind(to)
    .execute(db)
    .await?;

    Ok(())
}
