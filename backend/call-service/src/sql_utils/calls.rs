use sqlx::PgPool;
use tokio::sync::mpsc;
use topic_structs::{
    GroupCreatedEvent, GroupDeletedEvent, GroupUserAddedEvent, GroupUserRemovedEvent,
};
use tracing::debug;

use crate::{appstate::AppStateMessage, room::Room};

pub async fn create_group(pool: &PgPool, event: GroupCreatedEvent) {
    sqlx::query(
        "
        INSERT INTO rooms (id) VALUES ($1)
        ",
    )
    .bind(&event.group_id)
    .execute(pool)
    .await
    .ok();

    //TODO! (lamoara) Change this to proper bulk insert
    for user_id in event.member_ids {
        let event = GroupUserAddedEvent {
            group_id: event.group_id.clone(),
            user_id,
        };

        add_user_to_group(pool, event).await;
    }
    add_user_to_group(
        pool,
        GroupUserAddedEvent {
            group_id: event.group_id.clone(),
            user_id: event.owner_id,
        },
    )
    .await;
}

pub async fn delete_group(pool: &PgPool, event: GroupDeletedEvent) {
    sqlx::query(
        "
        DELETE FROM rooms WHERE id = $1
        ",
    )
    .bind(&event.group_id)
    .execute(pool)
    .await
    .ok();

    sqlx::query(
        "
        REMOVE FROM in_room WHERE room_id = $1
        ",
    )
    .bind(event.group_id)
    .execute(pool)
    .await
    .ok();
}

pub async fn add_user_to_group(pool: &PgPool, event: GroupUserAddedEvent) {
    sqlx::query(
        "
        INSERT INTO users (id) VALUES ($1)
        ",
    )
    .bind(&event.user_id)
    .execute(pool)
    .await
    .ok();

    sqlx::query(
        "
        INSERT INTO in_room (room_id, user_id) VALUES ($1, $2)
        ",
    )
    .bind(event.group_id)
    .bind(event.user_id)
    .execute(pool)
    .await
    .ok();
}

pub async fn remove_user_from_group(pool: &PgPool, event: GroupUserRemovedEvent) {
    sqlx::query(
        "
            DELETE FROM in_room WHERE room_id = $1 AND user_id = $2
        ",
    )
    .bind(event.group_id)
    .bind(event.user_id)
    .execute(pool)
    .await
    .ok();
}

pub async fn get_room_from_db(
    db: &PgPool,
    room_id: &str,
    sender: mpsc::Sender<AppStateMessage>,
) -> sqlx::Result<Room> {
    debug!("Searching for db room: {}", room_id);
    let users: Vec<String> = sqlx::query_scalar(
        "
        SELECT u.id
        FROM rooms r
        JOIN in_room ir ON ir.room_id = r.id
        JOIN users u ON ir.user_id = u.id
        WHERE r.id = $1
    ",
    )
    .bind(room_id)
    .fetch_all(db)
    .await?;

    debug!("Room fetched from database with users: {:?}", users);

    Ok(Room::new(users, sender, room_id.to_string()).await)
}
