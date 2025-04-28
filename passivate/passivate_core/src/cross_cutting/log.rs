use chrono::Utc;

use passivate_delegation::Tx;

use super::LogEvent;

#[mockall::automock]
pub trait Log {
    fn info(&self, message: &str);
}

pub fn stub_log() -> Box<MockLog> {
    let mut log = MockLog::new();
    log.expect_info().returning(|_|{});

    Box::new(log)
}

pub struct ChannelLog {
    sender: Tx<LogEvent>
}

impl ChannelLog {
    pub fn new(sender: Tx<LogEvent>) -> Self {
        Self { sender }
    }
}

impl Clone for ChannelLog {
    fn clone(&self) -> Self {
        Self { sender: self.sender.clone() }
    }
}

impl Log for ChannelLog {
    fn info(&self, message: &str) {
        self.sender.send(LogEvent { message: message.to_string(), timestamp: Utc::now() });
    }
}
