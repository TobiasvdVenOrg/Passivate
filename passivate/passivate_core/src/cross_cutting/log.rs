use std::sync::mpsc::Sender;

use chrono::Utc;

use super::LogEvent;

#[mockall::automock]
pub trait Log {
    fn info(&self, message: &str);
}

pub fn mock_log() -> Box<MockLog> {
    let mut log = MockLog::new();
    log.expect_info().returning(|_|{});

    Box::new(log)
}

#[derive(Clone)]
pub struct ChannelLog {
    sender: Sender<LogEvent>
}

impl ChannelLog {
    pub fn new(sender: Sender<LogEvent>) -> Self {
        Self { sender }
    }
}

impl Log for ChannelLog {
    fn info(&self, message: &str) {
        let _ = self.sender.send(LogEvent { message: message.to_string(), timestamp: Utc::now() });
    }
}
