use axum::{
    extract::{Path, Request},
    http::Method,
};
use tracing::info;

use crate::types::{AppError, AppJson};

pub(crate) async fn handler(
    method: Method,
    Path(path): Path<String>,
    req: Request,
) -> Result<AppJson<&'static str>, AppError> {
    Ok(AppJson("OK"))
}
