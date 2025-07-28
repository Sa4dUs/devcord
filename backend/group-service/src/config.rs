use sqlx::{PgPool, postgres::PgPoolOptions};
use std::env;

pub async fn init_db() -> anyhow::Result<PgPool> {
    let db_url = env::var("GROUP_DATABASE_URL")?;
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

    Ok(pool)
}
