use serde_json::from_slice;
use smol::stream::StreamExt;
use std::{env::var, sync::Arc, time::Duration};

use anyhow::{Result, anyhow};
use axum::{
    Router,
    http::{HeaderValue, Method, header},
    routing::get,
};
use dashmap::DashMap;
use fluvio::{
    FluvioConfig, Offset, consumer::ConsumerConfigExtBuilder, metadata::topic::TopicSpec,
};
use sqlx::{PgPool, postgres::PgPoolOptions};
use tokio::{
    net::TcpListener,
    sync::{Mutex, mpsc},
};
use topic_structs::GroupEvent;
use tower_http::{cors::CorsLayer, trace::TraceLayer};
use tracing::{debug, info};
use tracing_subscriber::{EnvFilter, fmt, layer::SubscriberExt, util::SubscriberInitExt};

use crate::{
    room::Room,
    sql_utils::calls::{add_user_to_group, create_group, delete_group, remove_user_from_group},
    user_ws::handle_upgrade,
};

pub type RoomID = String;
#[derive(Debug, Clone)]
pub enum AppStateMessage {
    RoomClosed(RoomID),
    AppClosed,
}

#[derive(Clone)]
pub struct AppState {
    pub rooms: Arc<DashMap<String, Arc<Mutex<Room>>>>,
    pool: PgPool,
    pub sender: mpsc::Sender<AppStateMessage>,
}

impl AppState {
    async fn new(db: PgPool) -> anyhow::Result<AppState> {
        let rooms: Arc<DashMap<String, Arc<Mutex<Room>>>> = Default::default();
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

            //TODO! (lamoara) clean up and change this to be able to close it without disconecting fluvio
            tokio::spawn(async move {
                while let Some(Ok(record)) = consumer_stream.next().await {
                    let Ok(event) = from_slice::<GroupEvent>(record.value()) else {
                        return; //TODO! This shouldnt break :D
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

async fn get_room_from_db(
    db: &PgPool,
    room_id: &str,
    sender: mpsc::Sender<AppStateMessage>,
) -> sqlx::Result<Room> {
    debug!("Searching for db room: {}", room_id);
    let users: Vec<String> = sqlx::query_scalar(
        "
        SELECT u.id
        FROM rooms r
        JOIN in_room ir ON ir.room_id = r.id
        JOIN users u ON ir.user_id = u.id
        WHERE r.id = $1
    ",
    )
    .bind(room_id)
    .fetch_all(db)
    .await?;

    debug!("Room Users: {:?}", users);

    Ok(Room::new(users, sender, room_id.to_string()).await)
}

async fn init(db: &PgPool) {
    sqlx::query(
        "
        CREATE TABLE IF NOT EXISTS users (
        id TEXT PRIMARY KEY
        )
    ",
    )
    .execute(db)
    .await
    .unwrap();

    sqlx::query(
        "
        CREATE TABLE IF NOT EXISTS rooms (
        id TEXT PRIMARY KEY
        )
    ",
    )
    .execute(db)
    .await
    .unwrap();

    sqlx::query(
        "
        CREATE TABLE IF NOT EXISTS in_room (
        room_id TEXT,
        user_id TEXT,
        CONSTRAINT in_room_pkey PRIMARY KEY (room_id, user_id)
        )
    ",
    )
    .execute(db)
    .await
    .unwrap();
}

pub async fn app() -> anyhow::Result<(Router, mpsc::Sender<AppStateMessage>)> {
    let trace_layer = TraceLayer::new_for_http();

    tracing_subscriber::registry()
        .with(fmt::layer())
        .with(EnvFilter::from_default_env())
        .init();

    let origins: Vec<HeaderValue> = var("CORS_ORIGIN")
        .expect("CORS_ORIGIN env not set")
        .split(",")
        .map(|e| e.trim().parse::<HeaderValue>())
        .collect::<Result<_, _>>()?;

    let cors_layer = CorsLayer::new()
        .allow_origin(origins)
        .allow_credentials(true)
        .allow_methods([Method::GET, Method::POST])
        .allow_headers([
            header::CONTENT_TYPE,
            header::AUTHORIZATION,
            header::ACCESS_CONTROL_ALLOW_ORIGIN,
        ]);

    let max_conns: u32 = var("DB_MAX_CONNECTIONS")
        .unwrap_or("1".to_owned())
        .parse()
        .expect("DB_MAX_CONNECTIONS must be a number");

    let db_timeout: u64 = var("DB_POOL_TIMEOUT_SECS")
        .unwrap_or("10".to_owned())
        .parse()
        .expect("DB_POOL_TIMEOUT_SECS must be a number");

    let db = PgPoolOptions::new()
        .max_connections(max_conns)
        .acquire_timeout(Duration::from_secs(db_timeout))
        .connect(
            var("CALL_DATABASE_URL")
                .expect("CALL_DATABASE_URL env not set")
                .trim(),
        )
        .await?;

    init(&db).await;

    let app_state = Arc::new(AppState::new(db).await?);
    let sender = app_state.sender.clone();
    let app = Router::new()
        .route("/health", get(|| async { "Viva el imperio Mongol!" }))
        .route("/", get(handle_upgrade))
        .layer(trace_layer)
        .layer(cors_layer)
        .with_state(app_state);

    Ok((app, sender))
}

pub async fn run(app: Router, sender: mpsc::Sender<AppStateMessage>) -> anyhow::Result<()> {
    let listener = TcpListener::bind("0.0.0.0:3000").await.unwrap();

    info!("Server running on 0.0.0.0:3000");

    axum::serve(listener, app).await?;

    sender.send(AppStateMessage::AppClosed).await?;
    Ok(())
}
