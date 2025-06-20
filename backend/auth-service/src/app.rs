use dotenvy::dotenv;
use sqlx::postgres::PgPoolOptions;
use std::env;

pub async fn run() -> Result<(), sqlx::Error> {
    dotenv().ok(); // I think this might not be necessary if we are using compose

    let database_url =
        env::var("AUTH_DATABASE_URL").expect("AUTH_DATABASE_URL debe estar definido en .env");

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await?;

    sqlx::migrate!().run(&pool).await?;

    println!("Database connected");

    Ok(())
}
