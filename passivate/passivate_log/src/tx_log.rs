use passivate_delegation::Tx;

use crate::log_message::LogMessage;

pub struct TxLog
{
    tx: Tx<LogMessage>
}

impl TxLog
{
    pub fn new(tx: Tx<LogMessage>) -> Self
    {
        Self { tx }
    }
}

impl log::Log for TxLog
{
    fn enabled(&self, _metadata: &log::Metadata) -> bool
    {
        true
    }

    fn log(&self, record: &log::Record)
    {
        self.tx.send(LogMessage::new(format!("{} - {}", record.level(), record.args())));
    }

    fn flush(&self) {}
}
