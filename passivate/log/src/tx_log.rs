use passivate_delegation::tx_rx::Tx;

use crate::log_message::LogMessage;

pub trait LogMessageTx = Tx<LogMessage> + Send + Sync + 'static;

pub struct TxLog<TTx: LogMessageTx>
{
    tx: TTx
}

impl<TTx: LogMessageTx> TxLog<TTx>
{
    pub fn new(tx: TTx) -> Self
    {
        Self { tx }
    }
}

impl<TTx: LogMessageTx> log::Log for TxLog<TTx>
{
    fn enabled(&self, _metadata: &log::Metadata) -> bool
    {
        true
    }

    fn log(&self, record: &log::Record)
    {
        if record.metadata().target().starts_with("passivate")
        {
            let message = LogMessage::new(format!("{} - {}", record.level(), record.args()));
            println!("{} {}", message.timestamp, message.content);

            self.tx.send(message);
        }
    }

    fn flush(&self) {}
}
