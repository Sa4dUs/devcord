use std::fs;

use anyhow::Error;
use serde::Deserialize;

const CONFIG_FILE: &str = "config/config.toml";

#[derive(Debug, Clone, Default, Deserialize)]
pub(crate) struct Config {
    pub(crate) services: std::collections::HashMap<String, Service>,
}

#[derive(Debug, Clone, Deserialize)]
pub(crate) struct Service {
    pub(crate) instances: Vec<Instance>,
    pub(crate) routes: Vec<Route>,
}

#[derive(Debug, Clone, Deserialize)]
pub(crate) struct Instance(pub(crate) String);

#[derive(Debug, Clone, Deserialize)]
pub(crate) struct Route {
    pub(crate) path: String,
    pub(crate) allow_methods: Vec<String>,
    pub(crate) protected: bool,
}

pub(crate) fn load() -> Result<Config, Error> {
    load_from_path(CONFIG_FILE)
}

fn load_from_path(path: &str) -> Result<Config, Error> {
    let content = fs::read_to_string(path)?;
    let config: Config = toml::from_str(&content)?;
    Ok(config)
}
