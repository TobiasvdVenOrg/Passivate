use crate::bridge::{Bridge, HypPath};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CompilationMessageKind
{
    Info,
    Warning,
    Error
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CompilationMessage
{
    pub content: String,
    pub kind: CompilationMessageKind
}

impl HypPath for CompilationMessage
{
    type TId = String;

    fn path(&self) -> Self::TId
    {
        self.content.clone()
    }
}

impl CompilationMessage
{
    pub fn new_info(message: impl Into<String>) -> Self
    {
        Self {
            content: message.into(),
            kind: CompilationMessageKind::Info
        }
    }

    pub fn new_warning(message: impl Into<String>) -> Self
    {
        Self {
            content: message.into(),
            kind: CompilationMessageKind::Warning
        }
    }

    pub fn new_error(message: impl Into<String>) -> Self
    {
        Self {
            content: message.into(),
            kind: CompilationMessageKind::Error
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum HypSessionEvent<TBridge: Bridge>
{
    RunStarted,
    Output(TBridge::TOutput),
    HypExists(TBridge::THypInfo),
    HypRunning(TBridge::TId),
    HypCompleted(TBridge::TId),
    RunCompleted
}
