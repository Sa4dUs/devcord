pub async fn init(db: &sqlx::PgPool) -> anyhow::Result<()> {
    sqlx::query(
        "
        CREATE TABLE IF NOT EXISTS users (
        id TEXT PRIMARY KEY,
        username TEXT NOT NULL UNIQUE,
        hashed_password TEXT NOT NULL,
        email TEXT NOT NULL UNIQUE,
        telephone TEXT
        )
        ",
    )
    .execute(db)
    .await?;

    Ok(())
}
