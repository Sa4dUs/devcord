use std::{collections::HashMap, env::var, sync::Arc, time::Duration};

use anyhow::{Result, anyhow};
use axum::{
    Router,
    http::{HeaderValue, Method, header},
    routing::get,
};
use dashmap::DashMap;
use sqlx::{PgPool, postgres::PgPoolOptions, prelude::FromRow};
use tokio::{net::TcpListener, sync::Mutex};
use tower_http::{cors::CorsLayer, trace::TraceLayer};
use tracing::{debug, info};
use tracing_subscriber::{EnvFilter, fmt, layer::SubscriberExt, util::SubscriberInitExt};

use crate::{room::Room, user_ws::handle_upgrade};

#[derive(Clone)]
pub struct AppState {
    pub rooms: Arc<DashMap<String, Arc<Mutex<Room>>>>,
    pool: PgPool,
}

impl AppState {
    async fn new(db: PgPool) -> AppState {
        AppState {
            rooms: Default::default(),
            pool: db,
        }
    }

    pub async fn get_room(&self, room_id: &str) -> Result<Arc<Mutex<Room>>> {
        if let Some(room) = self.rooms.get(room_id) {
            return Ok(room.clone());
        }

        let room = get_room_from_db(&self.pool, room_id).await?;

        debug!("Room from database: {:?}", room);

        self.rooms
            .insert(room_id.to_string(), Arc::new(Mutex::new(room)));

        if let Some(room) = self.rooms.get_mut(room_id) {
            return Ok(room.clone());
        }

        Err(anyhow!("Room does not exist"))
    }
}

async fn get_room_from_db(db: &PgPool, room_id: &str) -> sqlx::Result<Room> {
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

    Ok(Room::new(users).await)
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

pub async fn app() -> anyhow::Result<Router> {
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

    let app_state = Arc::new(AppState::new(db).await);

    let app = Router::new()
        .route("/health", get(|| async { "Viva el imperio Mongol!" }))
        .route("/", get(handle_upgrade))
        .layer(trace_layer)
        .layer(cors_layer)
        .with_state(app_state);

    Ok(app)
}

pub async fn run(app: Router) -> anyhow::Result<()> {
    let listener = TcpListener::bind("0.0.0.0:3000").await.unwrap();

    info!("Server running on 0.0.0.0:3000");

    axum::serve(listener, app).await?;

    Ok(())
}
