use chrono::{DateTime, Utc};

#[derive(Clone)]
pub struct LogMessage
{
    pub content: String,
    pub timestamp: DateTime<Utc>
}

impl LogMessage
{
    pub fn new(content: impl Into<String>) -> Self
    {
        Self::new_with_timestamp(content.into(), Utc::now())
    }

    pub fn new_with_timestamp(content: impl Into<String>, timestamp: DateTime<Utc>) -> Self
    {
        Self {
            content: content.into(),
            timestamp
        }
    }
}
