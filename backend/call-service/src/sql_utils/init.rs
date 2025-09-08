use sqlx::PgPool;

pub(crate) async fn run(db: &PgPool) -> anyhow::Result<()> {
    sqlx::query(
        "
        CREATE TABLE IF NOT EXISTS users (
        id TEXT PRIMARY KEY
        )
    ",
    )
    .execute(db)
    .await?;

    sqlx::query(
        "
        CREATE TABLE IF NOT EXISTS rooms (
        id TEXT PRIMARY KEY
        )
    ",
    )
    .execute(db)
    .await?;

    sqlx::query(
        "
        CREATE TABLE IF NOT EXISTS in_room (
        room_id TEXT,
        user_id TEXT,
        CONSTRAINT in_room_pkey PRIMARY KEY (room_id, user_id)
        )
    ",
    )
    .execute(db)
    .await?;

    Ok(())
}
