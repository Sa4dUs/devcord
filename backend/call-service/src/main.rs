use call_service::app::{app, run};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let app = app().await?;
    run(app).await
}
