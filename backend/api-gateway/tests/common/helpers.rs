use axum::{Router, routing::get};

pub(crate) async fn launch_instance(port: usize) {
    let app = Router::new().route("/", get(|| async { "Hello World!" }));
    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{port}"))
        .await
        .unwrap();
    axum::serve(listener, app).await.unwrap();
}
