use axum::{Router, routing::get};
use crate::app;

#[tokio::main]
async fn main() {
    app::run();
}
