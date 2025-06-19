use user_service::app;

#[tokio::main]
async fn main() {
    app::run().await.unwrap();
}
