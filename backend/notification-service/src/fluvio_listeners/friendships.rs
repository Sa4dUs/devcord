use fluvio::{Fluvio, consumer::ConsumerStream};

pub async fn run<T>(listener: &T) -> anyhow::Result<()>
where
    T: ConsumerStream,
{
    Ok(())
}
