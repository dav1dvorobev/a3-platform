use chrono::{DateTime, Utc};
use std::sync::Arc;

#[derive(Clone)]
pub struct AppState {
    pub startup_timestamp: Arc<DateTime<Utc>>,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            startup_timestamp: Arc::new(Utc::now()),
        }
    }
}
