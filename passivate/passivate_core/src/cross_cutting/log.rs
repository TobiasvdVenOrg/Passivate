use chrono::Utc;
use passivate_delegation::Tx;

use super::LogEvent;

#[mockall::automock]
pub trait Log: Send
{
    fn info(&mut self, message: &str);
}

pub fn stub_log() -> Box<MockLog>
{
    let mut log = MockLog::new();
    log.expect_info().returning(|_| {});

    Box::new(log)
}

pub struct ChannelLog
{
    sender: Tx<LogEvent>
}

impl ChannelLog
{
    pub fn new(sender: Tx<LogEvent>) -> Self
    {
        Self { sender }
    }

    pub fn boxed(sender: Tx<LogEvent>) -> Box<Self>
    {
        Box::new(Self::new(sender))
    }
}

impl From<ChannelLog> for Box<Tx<LogEvent>>
{
    fn from(log: ChannelLog) -> Self
    {
        Box::new(log.sender)
    }
}

impl Log for ChannelLog
{
    fn info(&mut self, message: &str)
    {
        self.sender.send(LogEvent {
            message: message.to_string(),
            timestamp: Utc::now()
        });
    }
}
