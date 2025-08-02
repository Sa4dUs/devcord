use std::env::var;

use fluvio::{FluvioConfig, TopicProducer, metadata::topic::TopicSpec, spu::SpuSocketPool};
use sqlx::PgPool;

#[derive(Clone)]
pub struct AppState {
    pub db: PgPool,
    pub producer: TopicProducer<SpuSocketPool>,
}

impl AppState {
    pub async fn new(db: PgPool) -> anyhow::Result<Self> {
        let mut fluvio_config =
            FluvioConfig::new(var("FLUVIO_ADDR").expect("FLUVIO_ADDR env not set").trim());
        fluvio_config.use_spu_local_address = true;

        let fluvio = fluvio::Fluvio::connect_with_config(&fluvio_config).await?;

        let producer_topic = var("PRODUCER_TOPIC")
            .unwrap_or("message-events".to_owned())
            .trim()
            .to_string();

        let admin = fluvio.admin().await;

        let topics = admin
            .all::<TopicSpec>()
            .await
            .expect("Failed to list topics");

        let topic_names = topics
            .iter()
            .map(|topic| topic.name.clone())
            .collect::<Vec<String>>();

        if !topic_names.contains(&producer_topic) {
            let topic_spec = TopicSpec::new_computed(1, 1, None);
            admin
                .create(producer_topic.clone(), false, topic_spec)
                .await?;
        }

        let producer = fluvio.topic_producer(producer_topic).await?;

        Ok(Self { db, producer })
    }
}
