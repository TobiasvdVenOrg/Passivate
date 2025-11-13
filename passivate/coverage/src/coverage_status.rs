use crate::grcov::covdir_json::CovdirJson;

#[derive(Clone, PartialEq, Debug)]
pub enum CoverageStatus
{
    Disabled,
    Preparing,
    Running,
    Done(Box<CovdirJson>),
    Error(String)
}
