use std::{env, sync::Arc};

use async_std::stream::StreamExt;
use axum::extract::ws::Message;
use dashmap::DashMap;
use fluvio::{
    FluvioConfig, Offset, consumer::ConsumerConfigExtBuilder, metadata::topic::TopicSpec,
};
use serde::{Deserialize, Serialize};
use serde_json::{Error, from_slice, to_string};

use crate::app::ResponseSender;

#[derive(Serialize, Deserialize)]
struct NotificationJson<T>
where
    T: Serialize,
{
    pub header: String,
    pub info: T,
}

pub async fn run<T>(
    senders: Arc<DashMap<String, ResponseSender>>,
    addr: String,
    topic_env: &str,
    obj_prefix: &str,
) -> anyhow::Result<()>
where
    T: for<'a> Deserialize<'a> + Serialize,
{
    let topic = env::var(topic_env).unwrap_or_else(|_| panic!("Topic env not set: {topic_env}"));

    let mut fluvio_config = FluvioConfig::new(addr);
    fluvio_config.use_spu_local_address = true;

    let fluvio = fluvio::Fluvio::connect_with_config(&fluvio_config).await?;

    let admin = fluvio.admin().await;

    let topics = admin
        .all::<TopicSpec>()
        .await
        .expect("Failed to list topics");
    let topic_names = topics
        .iter()
        .map(|topic| topic.name.clone())
        .collect::<Vec<String>>();

    if !topic_names.contains(&topic.to_string()) {
        let topic_spec = TopicSpec::new_computed(1, 1, None);
        admin.create(topic.to_string(), false, topic_spec).await?;
    }

    let consumer_config = ConsumerConfigExtBuilder::default()
        .topic(topic)
        .offset_start(Offset::beginning())
        .build()
        .expect("Failed to build consumer config");

    let mut listener = fluvio.consumer_with_config(consumer_config).await?;

    while let Some(Ok(record)) = listener.next().await {
        let parse_result: Result<T, Error> = from_slice(record.value());

        let Ok(obj) = parse_result else {
            continue;
        };

        let notification: NotificationJson<T> = NotificationJson {
            header: obj_prefix.to_string(),
            info: obj,
        };

        let Ok(response) = to_string(&notification) else {
            continue;
        };

        let Some(record_key) = record.key() else {
            continue;
        };

        let Ok(key) = std::str::from_utf8(record_key) else {
            continue;
        };

        let Some(sender) = senders.get(key) else {
            continue;
        };

        sender.send(Message::text(response)).await.ok();
    }

    Ok(())
}
