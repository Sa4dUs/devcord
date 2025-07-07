use crate::api_utils::structs::{
    PrivateBlocked, PrivateFriendRequest, PrivateFriendship, PrivateUser, PublicBlocked,
    PublicFriendRequestReceived, PublicFriendRequestSent, PublicUser,
};

//--------------------GETTERS--------------------

pub async fn get_public_user(id: &str, db: &sqlx::PgPool) -> Option<PublicUser> {
    sqlx::query_as(
        "
        SELECT username, created_at 
        FROM users 
        WHERE id = $1
    ",
    )
    .bind(id)
    .fetch_one(db)
    .await
    .ok()
}

pub async fn get_private_user(username: &str, db: &sqlx::PgPool) -> Option<PrivateUser> {
    sqlx::query_as(
        "
        SELECT id, username, created_at 
        FROM users 
        WHERE username = $1
    ",
    )
    .bind(username)
    .fetch_one(db)
    .await
    .ok()
}

pub async fn get_private_friendship(
    from_user_id: &str,
    to_user_id: &str,
    db: &sqlx::PgPool,
) -> Option<PrivateFriendship> {
    sqlx::query_as(
        "
        SELECT from_user_id, to_user_id, created_at
        FROM friendships
        WHERE from_user_id = $1 AND to_user_id = $2
    ",
    )
    .bind(from_user_id)
    .bind(to_user_id)
    .fetch_one(db)
    .await
    .ok()
}

pub async fn get_public_friend_requests_received(
    to_user_id: &str,
    from: i64,
    to: i64,
    db: &sqlx::PgPool,
) -> Option<Vec<PublicFriendRequestReceived>> {
    if from > to {
        return None;
    }

    let limit = (to - from).max(1);

    sqlx::query_as(
        "
        SELECT u.username, fr.status, fr.created_at 
        FROM friend_requests fr
        WHERE fr.to_user_id = $1
        JOIN users u
        ON u.id = fr.from_user_id 
        SORT BY fr.created_at
        LIMIT $2 OFFSET $3
    ",
    )
    .bind(to_user_id)
    .bind(limit)
    .bind(from)
    .fetch_all(db)
    .await
    .ok()
}

pub async fn get_public_friend_requests_sent(
    from_user_id: &str,
    from: i64,
    to: i64,
    db: &sqlx::PgPool,
) -> Option<Vec<PublicFriendRequestSent>> {
    if from > to {
        return None;
    }

    let limit = (to - from).max(1);

    sqlx::query_as(
        "
        SELECT u.username, fr.status, fr.created_at 
        FROM friend_requests fr
        WHERE fr.from_user_id = $1
        JOIN users u
        ON u.id = fr.to_user_id 
        SORT BY fr.created_at
        LIMIT $2 OFFSET $3
    ",
    )
    .bind(from_user_id)
    .bind(limit)
    .bind(from)
    .fetch_all(db)
    .await
    .ok()
}

pub async fn get_private_block(
    from_user_id: &str,
    to_user_id: &str,
    db: &sqlx::PgPool,
) -> Option<PrivateBlocked> {
    sqlx::query_as(
        "
        SELECT from_user_id, to_user_id, created_at
        FROM blocks
        WHERE from_user_id = $1 AND to_user_id = $2
    ",
    )
    .bind(from_user_id)
    .bind(to_user_id)
    .fetch_one(db)
    .await
    .ok()
}

pub async fn get_public_blocks(
    from_user_id: &str,
    from: i64,
    to: i64,
    db: &sqlx::PgPool,
) -> Option<Vec<PublicBlocked>> {
    sqlx::query_as(
        "
        SELECT u.username, b.created_at
        FROM blocks b
        WHERE b.from_user_id = $1
        JOIN users u ON u.id = b.to_user_id
        ORDER BY b.created_at DESC
        OFFSET $2
        LIMIT $3
    ",
    )
    .bind(from_user_id)
    .bind(from)
    .bind(to)
    .fetch_all(db)
    .await
    .ok()
}

pub async fn get_private_friend_request(
    from_user_id: &str,
    to_user_id: &str,
    db: &sqlx::PgPool,
) -> Option<PrivateFriendRequest> {
    sqlx::query_as(
        "
            SELECT from_user_id, to_user_id, state, created_at
            FROM friend_requests
            WHERE from_user_id = $1 AND to_user_id = $2
        ",
    )
    .bind(&from_user_id)
    .bind(&to_user_id)
    .fetch_one(db)
    .await
    .ok()
}

pub async fn get_undirected_private_friend_requests(
    from_user_id: &str,
    to_user_id: &str,
    db: &sqlx::PgPool,
) -> Option<Vec<PrivateFriendRequest>> {
    sqlx::query_as(
        "
            SELECT from_user_id, to_user_id, state, created_at
            FROM friend_requests
            WHERE (from_user_id = $1 AND to_user_id = $2) OR (from_user_id = $2 AND to_user_id = $1)
        ",
    )
    .bind(&from_user_id)
    .bind(&to_user_id)
    .fetch_all(db)
    .await
    .ok()
}

//--------------------INSERTS--------------------

pub async fn insert_user(user: PrivateUser, db: &sqlx::PgPool) -> anyhow::Result<()> {
    sqlx::query(
        "
            INSERT INTO users (id, username) VALUES ($1, $2)
        ",
    )
    .bind(user.id)
    .bind(user.username)
    .execute(db)
    .await?;

    Ok(())
}

pub async fn insert_friend_request(from: &str, to: &str, db: &sqlx::PgPool) -> anyhow::Result<()> {
    sqlx::query(
        "
        INSERT 
        INTO friend_requests (from_user_id, to_user_id) 
        VALUES ($1, $2)
    ",
    )
    .bind(from)
    .bind(to)
    .execute(db)
    .await?;

    Ok(())
}

pub async fn insert_friendship(
    a_user_id: &str,
    b_user_id: &str,
    db: &sqlx::PgPool,
) -> anyhow::Result<()> {
    sqlx::query(
        "
        INSERT 
        INTO friendships (from_user_id, to_user_id) 
        VALUES ($1, $2), ($2, $1)
    ",
    )
    .bind(&a_user_id)
    .bind(&b_user_id)
    .execute(db)
    .await?;

    Ok(())
}

pub async fn insert_block(block: PrivateBlocked, db: &sqlx::PgPool) -> anyhow::Result<()> {
    sqlx::query(
        "
        INSERT
        INTO blocks (from_user_id, to_user_id)
        VALUES ($1, $2)
    ",
    )
    .bind(block.from_user_id)
    .bind(block.to_user_id)
    .execute(db)
    .await?;

    Ok(())
}

//--------------------DELETE--------------------

pub async fn delete_friend_request(
    request: PrivateFriendRequest,
    db: &sqlx::PgPool,
) -> anyhow::Result<()> {
    sqlx::query(
        "
        DELETE 
        FROM friend_requests 
        WHERE (from_user_id = $1 AND to_user_id = $2) OR (from_user_id = $2 AND to_user_id = $1)
    ",
    )
    .bind(request.from_user_id)
    .bind(request.to_user_id)
    .execute(db)
    .await?;

    Ok(())
}

pub async fn delete_friendship(
    friendship: PrivateFriendship,
    db: &sqlx::PgPool,
) -> anyhow::Result<()> {
    sqlx::query(
        "
        DELETE 
        FROM friendships 
        WHERE (from_user_id = $1 AND to_user_id = $2) OR (from_user_id = $2 AND to_user_id = $1)
    ",
    )
    .bind(friendship.from_user_id)
    .bind(friendship.to_user_id)
    .execute(db)
    .await?;

    Ok(())
}

pub async fn delete_block(block: PrivateBlocked, db: &sqlx::PgPool) -> anyhow::Result<()> {
    sqlx::query(
        "
        DELETE 
        FROM blocks 
        WHERE from_user_id = $1 AND to_user_id = $2
    ",
    )
    .bind(block.from_user_id)
    .bind(block.to_user_id)
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
    friend_request: PrivateFriendRequest,
    db: &sqlx::PgPool,
) -> anyhow::Result<()> {
    sqlx::query(
        "
        UPDATE friend_requests
        SET state = $1, responded_at = CURRENT_TIMESTAMP
        WHERE from_user_id = $2 AND to_user_id = $3
    ",
    )
    .bind(friend_request.state.to_string())
    .bind(friend_request.from_user_id)
    .bind(friend_request.to_user_id)
    .execute(db)
    .await
    .ok();

    Ok(())
}
