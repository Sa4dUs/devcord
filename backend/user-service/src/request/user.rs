use std::sync::Arc;

use axum::{
    Json,
    extract::{Query, State},
    response::IntoResponse,
};
use axum_extra::either::Either::{self, E1, E2};

use crate::{
    api_utils::{
        responses,
        structs::{
            PublicUser, RequestUpdateProfile, RequestUpdateProfileEnum::Username,
            RequestUserProfile,
        },
    },
    app::AppState,
    jwt::Claims,
    sql_utils::calls::{get_private_user, get_public_user, update_user_username},
};

pub async fn update_profile(
    State(state): State<Arc<AppState>>,
    claims: Claims,
    Json(body): Json<RequestUpdateProfile>,
) -> impl IntoResponse {
    if get_public_user(&claims.user_id, &state.db).await.is_none() {
        return responses::USER_DOES_NOT_EXIST;
    }

    for (part, value) in body.query.iter() {
        let res = match part {
            Username => update_user_username(&claims.user_id, value, &state.db).await,
        };

        match res {
            Ok(_) => continue,
            Err(_) => return responses::DB_ERROR,
        }
    }

    responses::PROFILE_UPDATED
}

pub async fn get_user_info(
    State(state): State<Arc<AppState>>,
    Query(query): Query<RequestUserProfile>,
) -> Either<Json<PublicUser>, impl IntoResponse> {
    let user = match get_private_user(&query.user_username, &state.db).await {
        Some(e) => e,
        None => return E2(responses::USER_DOES_NOT_EXIST),
    };

    let info = match get_public_user(&user.id, &state.db).await {
        Some(e) => e,
        None => return E2(responses::USER_DOES_NOT_EXIST),
    };

    E1(Json(info))
}
