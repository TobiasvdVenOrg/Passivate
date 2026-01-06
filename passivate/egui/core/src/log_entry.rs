use passivate_log::log_message::LogMessage;

/// This 'LogEntry' caches a 'stringified' version of 'LogMessage' from '/log'
/// It's in /egui/core because this transformation is only for caching purposes
/// to re-use when re-rendering the log
pub struct LogEntry
{
    pub timestamp: String,
    pub message: String
}

impl From<LogMessage> for LogEntry
{
    fn from(value: LogMessage) -> Self
    {
        let timestamp_formatted = format!("{}", value.timestamp.format("%H:%M:%S"));
        Self {
            timestamp: timestamp_formatted,
            message: value.content
        }
    }
}
