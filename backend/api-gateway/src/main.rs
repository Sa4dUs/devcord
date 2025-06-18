#[tokio::main]
async fn main() -> anyhow::Result<()> {
    api_gateway::run().await
}
