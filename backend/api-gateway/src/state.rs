use crate::config::Config;

#[derive(Debug, Clone)]
pub(crate) struct AppState {
    pub(crate) config: Config,
}

impl AppState {
    pub(crate) fn new(config: Config) -> AppState {
        AppState { config }
    }
}
