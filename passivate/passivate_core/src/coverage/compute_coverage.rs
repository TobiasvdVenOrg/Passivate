use super::{CoverageError, CoverageStatus};


pub trait ComputeCoverage {
    fn compute_coverage(&self) -> Result<CoverageStatus, CoverageError>;
    fn clean_coverage_output(&self) -> Result<(), CoverageError>;
}
