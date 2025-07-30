use crate::passivate_grcov::CovdirJson;

#[derive(Clone, PartialEq)]
pub enum CoverageStatus
{
    Disabled,
    Preparing,
    Running,
    Done(Box<CovdirJson>),
    Error(String)
}
