use std::time::Instant;

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
}
