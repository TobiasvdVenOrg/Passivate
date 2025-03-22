use super::CoverageError;

#[derive(Clone)]
#[derive(Debug)]
pub enum CoverageStatus {
    Disabled,
    Preparing,
    Running,
    Done,
    Error(CoverageError)
}