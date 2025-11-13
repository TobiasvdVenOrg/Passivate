use chrono::{DateTime, Utc};

#[derive(Clone)]
pub struct LogMessage
{
    pub content: String,
    pub timestamp: DateTime<Utc>
}

impl LogMessage
{
    pub fn new(content: String) -> Self
    {
        Self::new_with_timestamp(content, Utc::now())
    }

    pub fn new_with_timestamp(content: String, timestamp: DateTime<Utc>) -> Self
    {
        Self {
            content,
            timestamp
        }
    }
}