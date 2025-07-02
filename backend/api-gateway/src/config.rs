use std::fs;

use anyhow::Error;
use serde::Deserialize;

#[cfg(not(test))]
const CONFIG_FILE: &str = "config/config.toml";
#[cfg(test)]
const CONFIG_FILE: &str = "config/config.example.toml";

#[derive(Debug, Clone, Default, Deserialize)]
pub struct Config {
    pub(crate) services: std::collections::HashMap<String, Service>,
}

#[derive(Debug, Clone, Deserialize)]
pub(crate) struct Service {
    pub(crate) instances: Vec<Instance>,
    // FIXME(Sa4dUs): Change this to a HashMap for O(1) time search
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

pub fn load() -> Result<Config, Error> {
    load_from_path(CONFIG_FILE)
}

pub fn load_from_path(path: &str) -> Result<Config, Error> {
    let content = fs::read_to_string(path)?;
    let config: Config = toml::from_str(&content)?;
    Ok(config)
}
