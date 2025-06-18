use axum::{
    Extension,
    extract::{Path, Request},
    http::Method,
};

use crate::{
    config::SharedConfig,
    middleware::parser::ParsedURI,
    types::{AppError, AppJson},
};

pub(crate) async fn handler(
    method: Method,
    Extension(config): Extension<SharedConfig>,
    Extension(ParsedURI { prefix, subpath }): Extension<ParsedURI>,
    req: Request,
) -> Result<AppJson<&'static str>, AppError> {
    Ok(AppJson("OK"))
}
