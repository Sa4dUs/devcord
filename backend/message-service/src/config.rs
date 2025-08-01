use sqlx::{PgPool, postgres::PgPoolOptions};
use std::env;

pub async fn init_db() -> anyhow::Result<PgPool> {
    let db_url = env::var("MESSAGE_DATABASE_URL")?;
    let max_connections = env::var("DB_MAX_CONNECTIONS")
        .unwrap_or_else(|_| "5".into())
        .parse::<u32>()?;
    let timeout = env::var("DB_POOL_TIMEOUT_SECS")
        .unwrap_or_else(|_| "10".into())
        .parse::<u64>()?;

    let pool = PgPoolOptions::new()
        .max_connections(max_connections)
        .acquire_timeout(std::time::Duration::from_secs(timeout))
        .connect(&db_url)
        .await?;

    sqlx::query(
        "CREATE TABLE IF NOT EXISTS messages (
    id UUID PRIMARY KEY,
    sender_id UUID NOT NULL,
    channel_id UUID NOT NULL,
    message TEXT NOT NULL,
    created_at TIMESTAMP DEFAULT now()
);
",
    )
    .execute(&pool)
    .await?;

    Ok(pool)
}
