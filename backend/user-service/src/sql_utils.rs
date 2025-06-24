pub async fn is_user_in_db(id: &str, db: &sqlx::PgPool) -> bool {
    sqlx::query("SELECT id FROM users WHERE id = $1")
        .bind(id)
        .fetch_one(db)
        .await
        .is_ok()
}

pub enum RequestState {
    None,
    Pending,
    Accepted,
    Rejected,
}

pub async fn check_request_in_db(from: &str, to: &str, db: &sqlx::PgPool) -> RequestState {
    let res = sqlx::query_as!(
        String,
        "SELECT status FROM friend_requests WHERE from_user_id = $1 AND to_user_id = $2"
    )
    .bind(from)
    .bind(to)
    .fetch_one(db)
    .await
    .ok();

    match res {
        None => RequestState::None,
        Some(state) if state.into() == "pending" => RequestState::Pending,
        Some(state) if state.into() == "accepted" => RequestState::Accepted,
        Some(state) if state.into() == "rejected" => RequestState::Rejected,
        _ => RequestState::None,
    }
}

pub async fn create_request(from: &str, to: &str, db: &sqlx::PgPool) -> anyhow::Result<()> {
    sqlx::query("INSERT INTO friend_requests (from_user_id, to_user_id) VALUES ($1, $2)")
        .bind(from)
        .bind(to)
        .execute(db)
        .await?;

    Ok(())
}
