use std::{fs, sync::Arc};

use anyhow::Error;
use serde::Deserialize;

pub(crate) type SharedConfig = Arc<Config>;

const CONFIG_FILE: &str = "config/config.toml";

#[derive(Debug, Deserialize)]
pub(crate) struct Config {
    services: std::collections::HashMap<String, Service>,
}

#[derive(Debug, Deserialize)]
struct Service {
    routes: Vec<Route>,
}

#[derive(Debug, Deserialize)]
struct Route {
    path: String,
    allow_methods: Vec<String>,
    protected: bool,
}

pub(crate) fn load() -> Result<Config, Error> {
    load_from_path(CONFIG_FILE)
}

fn load_from_path(path: &str) -> Result<Config, Error> {
    let content = fs::read_to_string(path)?;
    let config: Config = toml::from_str(&content)?;
    Ok(config)
}
