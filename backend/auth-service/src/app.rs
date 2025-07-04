use crate::register::register_user;
use crate::sign_in::sign_in_user;
use axum::{Extension, routing::post};
use dotenvy::dotenv;
use sqlx::postgres::PgPoolOptions;
use std::env;
use std::sync::Arc;

pub async fn run() -> Result<(), sqlx::Error> {
    dotenv().ok();

    let database_url =
        env::var("AUTH_DATABASE_URL").expect("AUTH_DATABASE_URL should be defined in .env");

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await?;

    sqlx::migrate!().run(&pool).await?;

    let shared_pool = Arc::new(pool);

    println!("Database connected");

    let app = axum::Router::new()
        .route("/register", post(register_user))
        .route("/sign_in", post(sign_in_user))
        .route("/", axum::routing::get(|| async { "Hello, world!" }))
        .layer(Extension(shared_pool.clone()));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    axum::serve(listener, app).await.unwrap();

    Ok(())
}
