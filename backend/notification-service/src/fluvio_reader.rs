use async_std::stream::StreamExt;
use axum::extract::ws::Message;
use fluvio::consumer::ConsumerStream;
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
    mut listener: Box<dyn ConsumerStream>,
    sender: ResponseSender,
    obj_prefix: String,
) -> anyhow::Result<()>
where
    T: for<'a> Deserialize<'a> + Serialize,
{
    while let Some(Ok(record)) = listener.next().await {
        let parse_result: Result<T, Error> = from_slice(record.value());

        let Ok(obj) = parse_result else {
            continue;
        };

        let notification: NotificationJson<T> = NotificationJson {
            header: obj_prefix.clone(),
            info: obj,
        };

        let Ok(response) = to_string(&notification) else {
            continue;
        };

        if sender.send(Message::text(response)).await.is_err() {
            break;
        }
    }

    Ok(())
}
