#[cfg(test)]
use mockall::*;

use super::{CoverageError, CoverageStatus};

#[cfg_attr(test, automock)]
pub trait ComputeCoverage {
    fn compute_coverage(&self) -> Result<CoverageStatus, CoverageError>;
    fn clean_coverage_output(&self) -> Result<(), CoverageError>;
}
