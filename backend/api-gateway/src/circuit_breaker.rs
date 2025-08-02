use std::time::{Duration, Instant};

use crate::config::CircuitBreakerConfig;

#[derive(Debug)]
pub enum CircuitState {
    Closed,
    Open { retry_after: Instant },
    HalfOpen,
}

#[derive(Debug)]
pub struct CircuitBreaker {
    pub failures: u32,
    pub state: CircuitState,
    pub last_checked: Instant,
    pub config: CircuitBreakerConfig,
}

impl Default for CircuitBreaker {
    fn default() -> Self {
        Self {
            failures: Default::default(),
            state: CircuitState::HalfOpen,
            last_checked: Instant::now(),
            config: CircuitBreakerConfig::default(),
        }
    }
}

impl CircuitBreaker {
    pub fn with_config(config: CircuitBreakerConfig) -> Self {
        Self {
            config,
            ..Default::default()
        }
    }
    pub fn register_failure(&mut self) {
        self.failures += 1;
        self.last_checked = Instant::now();
        self.state = CircuitState::Open {
            retry_after: Instant::now() + Duration::from_secs(self.config.open_window_seconds),
        };
    }

    pub fn register_success(&mut self) {
        self.failures = 0;
        self.state = CircuitState::Closed;
        self.last_checked = Instant::now();
    }
}
