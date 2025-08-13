use chrono::Utc;
use passivate_delegation::Tx;

use super::LogEvent;

#[mockall::automock]
pub trait Log: Send
{
    fn info(&mut self, message: &str);
}

pub struct TxLog<TTx>
{
    sender: TTx
}

impl<TTx> TxLog<TTx>
{
    pub fn new(sender: TTx) -> Self
    {
        Self { sender }
    }
}

impl<TTx> Log for TxLog<TTx>
where
    TTx: Tx<LogEvent> + Send
{
    fn info(&mut self, message: &str)
    {
        self.sender.send(LogEvent {
            message: message.to_string(),
            timestamp: Utc::now()
        });
    }
}
