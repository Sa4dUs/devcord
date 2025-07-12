use async_std::stream::StreamExt;
use dotenvy::var;
use fluvio::{Fluvio, Offset, consumer::ConsumerConfigExtBuilder};
use serde_json::from_slice;
use topic_structs::UserCreated;

use crate::{
    api_utils::structs::PrivateUser,
    sql_utils::calls::{get_public_user, insert_user},
};

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
        let parse_result = from_slice::<UserCreated>(record.value());

        if let Ok(user_created) = &parse_result {
            let user = PrivateUser {
                id: user_created.id.clone(),
                username: user_created.username.clone(),
                created_at: None,
            };
            if get_public_user(&user.id, &db).await.is_some() {
                //TODO! User already exists, big time error
                continue;
            }

            if insert_user(user, &db).await.is_err() {
                //TODO! IDK, panic I guess
            }
        }
    }

    Ok(())
}
