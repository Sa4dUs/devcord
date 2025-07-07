use fluvio::TopicProducer;

#[derive(Clone)]
pub struct AppState {
    pub db: sqlx::PgPool,
    pub producer: TopicProducer,
}
