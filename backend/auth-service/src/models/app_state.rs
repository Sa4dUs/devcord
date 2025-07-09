use fluvio::{TopicProducer, spu::SpuSocketPool};

#[derive(Clone)]
pub struct AppState {
    pub db: sqlx::PgPool,
    pub producer: TopicProducerMono,
}

// `TopicProducer<S>` requires a generic parameter `S`
// where `S: std::marker::Send + std::marker::Sync + fluvio::spu::SpuPool + 'static`
// We can define define this type since, even though `TopicProducer` takes 1 generic param,
// in this crate, only one variant is being used, where `S: fluvio::spu::SpuSockerPool`
type TopicProducerMono = TopicProducer<SpuSocketPool>;
