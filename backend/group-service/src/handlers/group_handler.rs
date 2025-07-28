use axum::{
    Extension,
    extract::{Json, State},
    http::StatusCode,
};
use sqlx::PgPool;
use uuid::Uuid;

use crate::{middleware::auth::AuthUser, models::group::CreateGroupRequest};

pub async fn create_group(
    State(pool): State<PgPool>,
    Extension(AuthUser { user_id }): Extension<AuthUser>,
    Json(payload): Json<CreateGroupRequest>,
) -> Result<Json<Uuid>, StatusCode> {
    if payload.member_ids.is_empty() {
        return Err(StatusCode::BAD_REQUEST);
    }

    let group_id = Uuid::new_v4();
    let channel_id = Uuid::new_v4();

    let mut tx = pool
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

    for member_id in payload.member_ids.iter().chain(std::iter::once(&user_id)) {
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

    Ok(Json(group_id))
}
