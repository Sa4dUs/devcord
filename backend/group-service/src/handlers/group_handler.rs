use axum::{
    Extension,
    extract::{Json, Query, State},
    http::StatusCode,
    response::IntoResponse,
};
use serde::Deserialize;
use uuid::Uuid;

use crate::{
    middleware::auth::AuthUser,
    models::group::{AddUsersRequest, CreateGroupRequest, GroupInfo, RemoveUserRequest},
    state::AppState,
};
use topic_structs::{
    GroupCreatedEvent, GroupDeletedEvent, GroupUserAddedEvent, GroupUserRemovedEvent,
};

#[derive(Deserialize)]
pub struct GroupQueryParams {
    pub from: Option<usize>,
    pub to: Option<usize>,
}

pub async fn create_group(
    State(state): State<AppState>,
    Extension(AuthUser { user_id }): Extension<AuthUser>,
    Json(payload): Json<CreateGroupRequest>,
) -> Result<impl IntoResponse, StatusCode> {
    if payload.member_ids.is_empty() {
        return Err(StatusCode::BAD_REQUEST);
    }

    let group_id = Uuid::new_v4();
    let channel_id = Uuid::new_v4();

    let mut tx = state
        .db
        .begin()
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    sqlx::query("INSERT INTO groups (id, owner_id, channel_id) VALUES ($1, $2, $3)")
        .bind(group_id)
        .bind(user_id)
        .bind(channel_id)
        .execute(&mut *tx)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let member_ids: Vec<Uuid> = payload
        .member_ids
        .iter()
        .chain(std::iter::once(&user_id))
        .cloned()
        .collect();

    for member_id in &member_ids {
        sqlx::query("INSERT INTO group_members (group_id, user_id) VALUES ($1, $2)")
            .bind(group_id)
            .bind(member_id)
            .execute(&mut *tx)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    }

    tx.commit()
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let event = GroupCreatedEvent {
        group_id: group_id.to_string(),
        owner_id: user_id.to_string(),
        channel_id: channel_id.to_string(),
        member_ids: member_ids.iter().map(|id| id.to_string()).collect(),
    };

    let event_bytes = serde_json::to_vec(&event).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    state
        .producer
        .send(group_id.to_string().as_str(), event_bytes)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    for member_id in member_ids {
        let event = GroupUserAddedEvent {
            group_id: group_id.to_string(),
            user_id: member_id.to_string(),
        };

        let event_bytes =
            serde_json::to_vec(&event).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
        state
            .producer
            .send(member_id.to_string().as_str(), event_bytes)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    }

    Ok((StatusCode::CREATED, Json(group_id)))
}

pub async fn add_users_to_group(
    State(state): State<AppState>,
    Extension(AuthUser { user_id }): Extension<AuthUser>,
    axum::extract::Path(group_id): axum::extract::Path<Uuid>,
    Json(payload): Json<AddUsersRequest>,
) -> Result<impl IntoResponse, StatusCode> {
    let owner: (Uuid,) = sqlx::query_as("SELECT owner_id FROM groups WHERE id = $1")
        .bind(group_id)
        .fetch_one(&state.db)
        .await
        .map_err(|_| StatusCode::NOT_FOUND)?;

    if owner.0 != user_id {
        return Err(StatusCode::FORBIDDEN);
    }

    let mut tx = state
        .db
        .begin()
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    for uid in &payload.user_ids {
        sqlx::query(
            "INSERT INTO group_members (group_id, user_id) VALUES ($1, $2) ON CONFLICT DO NOTHING",
        )
        .bind(group_id)
        .bind(uid)
        .execute(&mut *tx)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    }

    tx.commit()
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    for user_id in payload.user_ids {
        let event = GroupUserAddedEvent {
            group_id: group_id.to_string(),
            user_id: user_id.to_string(),
        };
        let event_bytes =
            serde_json::to_vec(&event).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
        state
            .producer
            .send(user_id.to_string().as_str(), event_bytes)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    }

    Ok(StatusCode::NO_CONTENT)
}

pub async fn delete_group(
    State(state): State<AppState>,
    Extension(AuthUser { user_id }): Extension<AuthUser>,
    axum::extract::Path(group_id): axum::extract::Path<Uuid>,
) -> Result<impl IntoResponse, StatusCode> {
    let owner: (Uuid,) = sqlx::query_as("SELECT owner_id FROM groups WHERE id = $1")
        .bind(group_id)
        .fetch_one(&state.db)
        .await
        .map_err(|_| StatusCode::NOT_FOUND)?;

    if owner.0 != user_id {
        return Err(StatusCode::FORBIDDEN);
    }

    let rows: Vec<(Uuid,)> =
        sqlx::query_as("SELECT user_id FROM group_members WHERE group_id = $1")
            .bind(group_id)
            .fetch_all(&state.db)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let member_ids: Vec<Uuid> = rows.into_iter().map(|(id,)| id).collect();

    sqlx::query("DELETE FROM groups WHERE id = $1")
        .bind(group_id)
        .execute(&state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let event = GroupDeletedEvent {
        group_id: group_id.to_string(),
        owner_id: user_id.to_string(),
        member_ids: member_ids.iter().map(|id| id.to_string()).collect(),
    };
    let event_bytes = serde_json::to_vec(&event).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    state
        .producer
        .send(group_id.to_string().as_str(), event_bytes)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    for member_id in member_ids {
        let removed_event = GroupUserRemovedEvent {
            group_id: group_id.to_string(),
            user_id: member_id.to_string(),
        };
        let event_bytes =
            serde_json::to_vec(&removed_event).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
        state
            .producer
            .send(member_id.to_string().as_str(), event_bytes)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    }

    Ok(StatusCode::NO_CONTENT)
}

pub async fn list_user_groups(
    State(state): State<AppState>,
    Extension(AuthUser { user_id }): Extension<AuthUser>,
    Query(params): Query<GroupQueryParams>,
) -> Result<impl IntoResponse, StatusCode> {
    let (offset, limit) = match (params.from, params.to) {
        (Some(from), Some(to)) if to > from => (from as i64, (to - from) as i64),
        (Some(from), None) => (from as i64, 10),
        (None, Some(to)) => (0, to as i64),
        _ => (0, 10),
    };

    let groups = sqlx::query_as::<_, GroupInfo>(
        "SELECT g.id, g.owner_id, g.channel_id, g.created_at
         FROM groups g
         JOIN group_members gm ON g.id = gm.group_id
         WHERE gm.user_id = $1
         ORDER BY g.created_at DESC
         OFFSET $2 LIMIT $3",
    )
    .bind(user_id)
    .bind(offset)
    .bind(limit)
    .fetch_all(&state.db)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(groups))
}

pub async fn remove_user_from_group(
    State(state): State<AppState>,
    Extension(AuthUser { user_id }): Extension<AuthUser>,
    axum::extract::Path(group_id): axum::extract::Path<Uuid>,
    Json(payload): Json<RemoveUserRequest>,
) -> Result<impl IntoResponse, StatusCode> {
    let owner: (Uuid,) = sqlx::query_as("SELECT owner_id FROM groups WHERE id = $1")
        .bind(group_id)
        .fetch_one(&state.db)
        .await
        .map_err(|_| StatusCode::NOT_FOUND)?;

    if owner.0 != user_id || payload.user_id == user_id {
        return Err(StatusCode::FORBIDDEN);
    }

    sqlx::query("DELETE FROM group_members WHERE group_id = $1 AND user_id = $2")
        .bind(group_id)
        .bind(payload.user_id)
        .execute(&state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let event = GroupUserRemovedEvent {
        group_id: group_id.to_string(),
        user_id: payload.user_id.to_string(),
    };
    let event_bytes = serde_json::to_vec(&event).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    state
        .producer
        .send(payload.user_id.to_string().as_str(), event_bytes)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(StatusCode::NO_CONTENT)
}
