use passivate_model_bridge::{Bridge, OutputReport};

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
pub struct ConsoleOutput
{
    pub content: String,
    pub kind: ConsoleOutputKind
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ConsoleOutputKind
{
    StdOut,
    StdErr
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum HypSessionEvent<TBridge: Bridge>
{
    RunStarted,
    Output(OutputReport<TBridge>),
    HypExists(TBridge::HypInfo),
    HypRunning(TBridge::Id),
    HypCompleted(TBridge::Id),
    RunCompleted
}
