use std::time::SystemTime;


pub struct TraceEvent {
    pub message: String,
    pub timestamp: SystemTime
}

impl TraceEvent {
    pub fn new(message: &str) -> Self {
        Self { message: message.to_string(), timestamp: SystemTime::now() }
    }
}