use async_std::stream::StreamExt;
use bincode::{borrow_decode_from_slice, config::standard, error::DecodeError};
use dotenvy::var;
use fluvio::{Fluvio, Offset, consumer::ConsumerConfigExtBuilder};
use topic_structs::UserCreated;

pub async fn run(fluvio: Fluvio, db: sqlx::PgPool) -> anyhow::Result<()> {
    let consumer_config = ConsumerConfigExtBuilder::default()
        .topic(
            var("CONSUMER_TOPIC")
                .expect("CONSUMER_TOPIC env not set")
                .trim(),
        )
        .offset_start(Offset::beginning())
        .build()
        .expect("Failed to build consumer config");

    let mut consumer_stream = fluvio.consumer_with_config(consumer_config).await?;

    while let Some(Ok(record)) = consumer_stream.next().await {
        let parse_result: Result<(UserCreated, usize), DecodeError> =
            borrow_decode_from_slice(record.value(), standard());

        if let Ok((user_created, _)) = &parse_result {
            if sqlx::query(
                "
                SELECT id FROM users WHERE id = $1",
            )
            .bind(&user_created.id)
            .fetch_one(&db)
            .await
            .is_ok()
            {
                continue;
            }

            sqlx::query(
                "
                INSERT INTO users (id, username) VALUES ($1, $2)
            ",
            )
            .bind(&user_created.id)
            .bind(&user_created.username)
            .execute(&db)
            .await?;
        }
    }

    Ok(())
}
