use chrono::{DateTime, Utc};

#[derive(Clone)]
pub struct LogEvent {
    pub message: String,
    pub timestamp: DateTime<Utc>
}

impl LogEvent {
    pub fn new(message: &str) -> Self {
        Self::new_with_timestamp(message, Utc::now())
    }

    pub fn new_with_timestamp(message: &str, timestamp: DateTime<Utc>) -> Self {
        Self { message: message.to_string(), timestamp }
    }
}