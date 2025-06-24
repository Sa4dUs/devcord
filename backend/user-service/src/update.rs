use std::{collections::HashMap, sync::Arc};

use anyhow::Ok;
use axum::{Json, extract::State, http::StatusCode, response::IntoResponse};
use bincode::{config::standard, encode_to_vec};
use fluvio::RecordKey;
use serde::{Deserialize, Serialize};
use topic_structs::UserUpdated;

use crate::{api_responses, app::AppState, sql_utils::is_user_in_db};

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
    if !is_user_in_db(&payload.id, &state.db).await {
        return api_responses::USER_DOES_NOT_EXIST;
    }

    for (i, query) in payload.queries.iter().enumerate() {
        if user_query(&payload.id, query, state.clone()).await.is_err() {
            //Sometinhg was updated, should inform fluvio
            if i > 0 {
                send_fluvio_update(&payload.id, state.clone()).await.ok();
            }

            return api_responses::ILLEGAL_QUERY;
        }
    }

    if let Err(_) = send_fluvio_update(&payload.id, state.clone()).await {
        return api_responses::FLUVIO_ERROR;
    }

    return api_responses::USER_UPDATED;
}

async fn user_query(
    id: &str,
    query: (&QueryTypes, &String),
    state: Arc<AppState>,
) -> anyhow::Result<()> {
    let query_type = query.0;
    let new_value = query.1;

    match query_type {
        QueryTypes::Username => {
            sqlx::query(
                "
                    UPDATE users
                    SET username = $2
                    WHERE id = $1
                ",
            )
            .bind(id)
            .bind(new_value)
            .execute(&state.db)
            .await?;
        }
        _ => (),
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
