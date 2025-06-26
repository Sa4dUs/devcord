use std::{collections::HashMap, sync::Arc};

use anyhow::Ok;
use axum::{Json, extract::State, response::IntoResponse};
use bincode::{config::standard, encode_to_vec};
use fluvio::RecordKey;
use serde::Deserialize;
use topic_structs::UserUpdated;

use crate::{
    api_utils::responses,
    app::AppState,
    sql_utils::calls::{get_user, update_user_username},
};

#[derive(Deserialize, PartialEq, Eq, Hash, Debug)]
pub enum QueryTypes {
    Username,
}

#[derive(Deserialize)]
pub struct UpdateQuery {
    pub id: String,
    pub queries: HashMap<QueryTypes, String>,
}

pub async fn update(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<UpdateQuery>,
) -> impl IntoResponse {
    if get_user(&payload.id, &state.db).await.is_err() {
        return responses::USER_DOES_NOT_EXIST;
    }

    for (i, query) in payload.queries.iter().enumerate() {
        if use_query(&payload.id, query, &state.db).await.is_err() {
            //Sometinhg was updated, should inform fluvio
            if i > 0 {
                send_fluvio_update(&payload.id, state.clone()).await.ok();
            }

            return responses::ILLEGAL_QUERY;
        }
    }

    if send_fluvio_update(&payload.id, state.clone())
        .await
        .is_err()
    {
        return responses::FLUVIO_ERROR;
    }

    responses::USER_UPDATED
}

#[allow(clippy::single_match)]
async fn use_query(
    id: &str,
    query: (&QueryTypes, &String),
    db: &sqlx::PgPool,
) -> anyhow::Result<()> {
    let query_type = query.0;
    let new_value = query.1;

    match query_type {
        QueryTypes::Username => update_user_username(id, new_value, db).await?,
    }
    Ok(())
}

async fn send_fluvio_update(id: &str, state: Arc<AppState>) -> anyhow::Result<()> {
    let cargo = UserUpdated { id: id.to_owned() };

    state
        .producer
        .send(RecordKey::NULL, encode_to_vec(cargo, standard())?)
        .await?;

    Ok(())
}
