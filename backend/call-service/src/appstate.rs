use serde_json::from_slice;
use std::{env::var, sync::Arc};
use tokio_stream::StreamExt;

use anyhow::{Result, anyhow};
use dashmap::DashMap;
use fluvio::{
    FluvioConfig, Offset, consumer::ConsumerConfigExtBuilder, metadata::topic::TopicSpec,
};
use sqlx::PgPool;
use tokio::sync::{Mutex, mpsc};
use topic_structs::GroupEvent;
use tracing::{debug, error};

use crate::{
    room::Room,
    sql_utils::calls::{
        add_user_to_group, create_group, delete_group, get_room_from_db, remove_user_from_group,
    },
};

type RoomMap = Arc<DashMap<String, Arc<Mutex<Room>>>>;

pub type RoomID = String;
#[derive(Debug, Clone)]
pub enum AppStateMessage {
    RoomClosed(RoomID),
    AppClosed,
}

#[derive(Clone)]
pub struct AppState {
    pub rooms: RoomMap,
    pool: PgPool,
    pub sender: mpsc::Sender<AppStateMessage>,
}

impl AppState {
    pub async fn new(db: PgPool) -> anyhow::Result<AppState> {
        let rooms: RoomMap = Default::default();
        let (tx, mut rx) = mpsc::channel(10);

        {
            let rooms = rooms.clone();
            tokio::spawn(async move {
                while let Some(msg) = rx.recv().await {
                    match msg {
                        AppStateMessage::RoomClosed(id) => {
                            rooms.remove(&id);
                        }
                        AppStateMessage::AppClosed => return,
                    }
                }

                debug!("Room manager stopped");
            });
        }

        {
            let db = db.clone();
            let mut fluvio_config =
                FluvioConfig::new(var("FLUVIO_ADDR").expect("FLUVIO_ADDR env not set").trim());
            fluvio_config.use_spu_local_address = true;

            let fluvio = fluvio::Fluvio::connect_with_config(&fluvio_config).await?;

            let group_event_topic = var("GROUP_EVENT_TOPIC")
                .unwrap_or("group-events".to_owned())
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

            if !topic_names.contains(&group_event_topic) {
                let topic_spec = TopicSpec::new_computed(1, 1, None);
                admin
                    .create(group_event_topic.clone(), false, topic_spec)
                    .await?;
            }

            let consumer_config = ConsumerConfigExtBuilder::default()
                .topic(group_event_topic)
                .offset_start(Offset::beginning())
                .build()
                .expect("Failed to build consumer config");

            let mut consumer_stream = fluvio.consumer_with_config(consumer_config).await?;

            tokio::spawn(async move {
                while let Some(Ok(record)) = consumer_stream.next().await {
                    let Ok(event) = from_slice::<GroupEvent>(record.value())
                        .map_err(|e| error!("Error parsing group event: {}", e))
                    else {
                        continue; //This shouldnt break, we asume its a weird thing and just log it
                    };

                    match event {
                        GroupEvent::GroupCreatedEvent(event) => create_group(&db, event).await,
                        GroupEvent::GroupDeletedEvent(event) => delete_group(&db, event).await,
                        GroupEvent::GroupUserAddedEvent(event) => {
                            add_user_to_group(&db, event).await
                        }
                        GroupEvent::GroupUserRemovedEvent(event) => {
                            remove_user_from_group(&db, event).await
                        }
                    }
                }

                debug!("Fluvio reader stopped");
            });
        }

        Ok(AppState {
            rooms,
            pool: db,
            sender: tx,
        })
    }

    pub async fn get_room(&self, room_id: &str) -> Result<Arc<Mutex<Room>>> {
        if let Some(room) = self.rooms.get(room_id) {
            return Ok(room.clone());
        }

        let room = get_room_from_db(&self.pool, room_id, self.sender.clone()).await?;

        debug!("Room from database: {:?}", room);

        self.rooms
            .insert(room_id.to_string(), Arc::new(Mutex::new(room)));

        if let Some(room) = self.rooms.get_mut(room_id) {
            return Ok(room.clone());
        }

        Err(anyhow!("Room does not exist"))
    }
}
