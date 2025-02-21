use super::CoverageError;


#[derive(Clone)]
#[derive(Debug)]
pub enum CoverageStatus {
    Disabled,
    Error(CoverageError)
}