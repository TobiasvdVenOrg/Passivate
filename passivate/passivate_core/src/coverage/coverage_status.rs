use crate::passivate_grcov::CovdirJson;

use super::CoverageError;

#[derive(Clone, Debug, PartialEq)]
pub enum CoverageStatus {
    Disabled,
    Preparing,
    Running,
    Done(Box<CovdirJson>),
    Error(CoverageError)
}