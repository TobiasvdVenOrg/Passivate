use crate::actors::Cancellation;

use super::{CoverageError, CoverageStatus};

#[mockall::automock]
pub trait ComputeCoverage {
    fn compute_coverage(&self, cancellation: Cancellation) -> Result<CoverageStatus, CoverageError>;
    fn clean_coverage_output(&self) -> Result<(), CoverageError>;
}
