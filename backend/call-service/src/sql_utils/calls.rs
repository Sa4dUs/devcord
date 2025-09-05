use sqlx::PgPool;
use topic_structs::{
    GroupCreatedEvent, GroupDeletedEvent, GroupUserAddedEvent, GroupUserRemovedEvent,
};

pub async fn create_group(pool: &PgPool, event: GroupCreatedEvent) {
    sqlx::query(
        "
        INSERT INTO rooms (id) VALUES $1
        ",
    )
    .bind(&event.group_id)
    .execute(pool)
    .await
    .unwrap();

    //TODO! (lamoara) Change this to proper bulk insert
    for user_id in event.member_ids {
        let event = GroupUserAddedEvent {
            group_id: event.group_id.clone(),
            user_id: user_id,
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
    .unwrap();

    sqlx::query(
        "
        REMOVE FROM in_room WHERE room_id = $1
        ",
    )
    .bind(event.group_id)
    .execute(pool)
    .await
    .unwrap();
}

pub async fn add_user_to_group(pool: &PgPool, event: GroupUserAddedEvent) {
    sqlx::query(
        "
        INSERT INTO users (id) VALUES $1
        ",
    )
    .bind(&event.user_id)
    .execute(pool)
    .await
    .ok();

    sqlx::query(
        "
        INSERT INTO in_room (room_id, user_id) VALUES $1, $2
        ",
    )
    .bind(event.group_id)
    .bind(event.user_id)
    .execute(pool)
    .await
    .unwrap();
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
    .unwrap();
}
