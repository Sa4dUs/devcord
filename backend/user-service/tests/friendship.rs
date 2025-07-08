use axum::{body::Body, http::Request};
use tower::ServiceExt;
use user_service::app::app;

#[tokio::test]
async fn add_friend() {
    let (app, fluvio, db) = app().await.unwrap();

    app.oneshot(
        Request::builder()
            .uri("/health")
            .body(Body::empty())
            .unwrap(),
    )
    .await
    .unwrap();
}
