use std::fs;

use anyhow::Error;
use dotenv::var;
use serde::Deserialize;

#[derive(Debug, Clone, Default, Deserialize)]
pub struct Config {
    pub(crate) services: std::collections::HashMap<String, Service>,
    #[serde(default)]
    pub(crate) rate_limit: RateLimitConfig,
}

#[derive(Debug, Clone, Deserialize)]
pub(crate) struct Service {
    pub(crate) instances: Vec<Instance>,
    pub(crate) routes: Vec<Route>,
    #[serde(default)]
    pub(crate) strategy: Strategy,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub(crate) enum Strategy {
    #[default]
    Random,
}

#[derive(Debug, Clone, Deserialize)]
pub(crate) struct Instance(pub(crate) String);

#[derive(Debug, Clone, Deserialize)]
pub(crate) struct Route {
    pub(crate) path: String,
    pub(crate) allow_methods: Vec<String>,
    pub(crate) protected: bool,
}

#[derive(Debug, Clone, Deserialize)]
pub struct RateLimitConfig {
    pub max_requests: u32,
    pub window_seconds: u64,
}

impl Default for RateLimitConfig {
    fn default() -> Self {
        Self {
            max_requests: 100,
            window_seconds: 60,
        }
    }
}

pub fn load() -> Result<Config, Error> {
    let config_path = var("CONFIG_PATH").unwrap_or("config/config.toml".to_string());
    load_from_path(&config_path)
}

pub fn load_from_path(path: &str) -> Result<Config, Error> {
    let content = fs::read_to_string(path)?;
    let config: Config = toml::from_str(&content)?;
    Ok(config)
}
