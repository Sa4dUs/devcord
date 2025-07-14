use auth_service::app;
use tracing_subscriber::EnvFilter;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing::info!("Starting auth service...");

    app::run().await?;

    Ok(())
}
