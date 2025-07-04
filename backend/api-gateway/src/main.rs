#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv::dotenv().ok();
    api_gateway::run().await
}
