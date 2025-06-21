use axum::{Router, routing::get};
use dotenvy::dotenv;
use sqlx::postgres::PgPoolOptions;
use std::env;

pub async fn run() -> Result<(), sqlx::Error> {
    dotenv().ok();

    let app = Router::new().route("/", get(|| async { "Hello, World!" }));

    let database_url =
        env::var("AUTH_DATABASE_URL").expect("AUTH_DATABASE_URL should be defined in .env");

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await?;

    sqlx::migrate!().run(&pool).await?;

    println!("Database connected");

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    axum::serve(listener, app).await.unwrap();

    Ok(())
}
