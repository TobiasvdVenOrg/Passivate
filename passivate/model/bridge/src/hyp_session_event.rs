use std::fmt::Display;

use crate::bridge::Bridge;
use crate::hyp_report::HypReport;
use crate::output_report::OutputReport;

#[derive(Debug, Clone, PartialEq, Eq, Display)]
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

impl Display for CompilationMessage
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
    {
        write!(f, "{}: {}", self.kind, self.content)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConsoleOutput
{
    pub content: String,
    pub kind: ConsoleOutputKind
}

impl ConsoleOutput
{
    pub fn new_stdout(content: impl Into<String>) -> Self
    {
        Self {
            content: content.into(),
            kind: ConsoleOutputKind::StdOut
        }
    }

    pub fn new_stderr(content: impl Into<String>) -> Self
    {
        Self {
            content: content.into(),
            kind: ConsoleOutputKind::StdErr
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ConsoleOutputKind
{
    StdOut,
    StdErr
}

#[derive(Debug)]
pub enum HypSessionEvent<TBridge: Bridge>
{
    RunStarted,
    Output(OutputReport<TBridge>),
    Hyp(HypReport<TBridge>),
    RunCompleted,
    RunError(TBridge::RunError)
}

impl<TBridge: Bridge> Display for HypSessionEvent<TBridge>
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
    {
        match self
        {
            HypSessionEvent::RunStarted => write!(f, "Run Started"),
            HypSessionEvent::Output(output_report) => write!(f, "Output: {}", output_report.id()),
            HypSessionEvent::Hyp(hyp_report) => write!(f, "Hyp: {}", hyp_report.hyp_info),
            HypSessionEvent::RunCompleted => write!(f, "Run Completed"),
            HypSessionEvent::RunError(_) => write!(f, "Run Error")
        }
    }
}

impl<TBridge: Bridge> PartialEq for HypSessionEvent<TBridge>
{
    fn eq(&self, other: &Self) -> bool
    {
        match (self, other)
        {
            (Self::Output(l0), Self::Output(r0)) => l0 == r0,
            (Self::Hyp(l0), Self::Hyp(r0)) => l0 == r0,
            (Self::RunError(_), Self::RunError(_)) => panic!("attempt to compare HypSessionEvent::RunError for equality"),
            _ => core::mem::discriminant(self) == core::mem::discriminant(other)
        }
    }
}

impl<TBridge: Bridge> Eq for HypSessionEvent<TBridge> {}
