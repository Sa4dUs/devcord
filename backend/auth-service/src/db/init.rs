pub async fn init(db: &sqlx::PgPool) -> anyhow::Result<()> {
    sqlx::query(
        "
        CREATE EXTENSION IF NOT EXISTS pgcrypto
        ",
    )
    .execute(db)
    .await?;

    sqlx::query(
        "
        CREATE TABLE IF NOT EXISTS users (
        id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
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
