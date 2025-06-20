use auth_service::app;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    app::run().await?; // ← aquí se propaga el Result correctamente
    Ok(())
}
