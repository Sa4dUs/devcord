use fluvio::TopicProducer;

#[derive(Clone)]
pub struct AppState<S> {
    pub db: sqlx::PgPool,
    pub producer: TopicProducer<S>,
}
