use crate::logoff::log_user_off;
use crate::models::app_state::AppState;
use crate::register::register_user;
use crate::sign_in::sign_in_user;

use axum::{Router, routing::post};
use dotenvy::dotenv;
use fluvio::Fluvio;
use sqlx::postgres::PgPoolOptions;
use std::{env, time::Duration};

pub async fn run() -> Result<(), sqlx::Error> {
    dotenv().ok();

    let database_url =
        env::var("AUTH_DATABASE_URL").expect("AUTH_DATABASE_URL must be set in .env");

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .acquire_timeout(Duration::from_secs(3))
        .connect(&database_url)
        .await?;

    sqlx::migrate!().run(&pool).await?;
    println!("Database connected");

    let fluvio = Fluvio::connect()
        .await
        .expect("Failed to connect to Fluvio");
    let producer = fluvio
        .topic_producer("auth")
        .await
        .expect("Failed to create topic producer");

    let state = AppState { db: pool, producer };

    let app = Router::new()
        .route("/register", post(register_user))
        .route("/sign_in", post(sign_in_user))
        .route("/log_off", post(log_user_off))
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();

    Ok(())
}
