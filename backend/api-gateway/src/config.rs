use std::fs;

use anyhow::Error;
use dotenv::var;
use serde::Deserialize;

#[derive(Debug, Clone, Default, Deserialize)]
pub struct Config {
    pub(crate) services: std::collections::HashMap<String, Service>,
    #[serde(default)]
    pub(crate) rate_limit: RateLimitConfig,
    #[serde(default)]
    pub(crate) circuit_breaker: CircuitBreakerConfig,
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

#[derive(Debug, Clone, Deserialize)]
pub struct CircuitBreakerConfig {
    pub failure_threshold: u32,
    pub open_window_seconds: u64,
}

impl Default for CircuitBreakerConfig {
    fn default() -> Self {
        Self {
            failure_threshold: 3,
            open_window_seconds: 30,
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

impl Config {
    pub(crate) fn get_route(&self, svc: &str, route: &str) -> Option<crate::config::Route> {
        // FIXME(Sa4dUs): Collapse if-let statements when using rust 2024 edition
        if let Some(svc) = self.services.get(svc) {
            if let Some(route) = svc.get_route(route) {
                return Some(route);
            }
        }
        None
    }
}

impl Service {
    pub(crate) fn get_route(&self, route: &str) -> Option<crate::config::Route> {
        self.routes
            .iter()
            .find(|r| r.path == route)
            .or_else(|| {
                self.routes.iter().find(|r| {
                    let regex = path_pattern_to_regex(&r.path);
                    regex.is_match(route)
                })
            })
            .cloned()
    }
}

// FIXME(Sa4dUs): Move this to a utils module
fn path_pattern_to_regex(path: &str) -> regex::Regex {
    let pattern = path
        .replace("-", r"\-")
        .replace(".", r"\.")
        .replace("{", "(?P<")
        .replace("}", ">[^/]+)");

    let full_pattern = format!("^{pattern}$");
    regex::Regex::new(&full_pattern).expect("Invalid route regex")
}
