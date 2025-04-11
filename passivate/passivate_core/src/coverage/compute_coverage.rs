use crate::delegation::Cancellation;

use super::{CoverageError, CoverageStatus};

#[mockall::automock]
pub trait ComputeCoverage {
    fn compute_coverage(&self, cancellation: Cancellation) -> Result<CoverageStatus, CoverageError>;
    fn clean_coverage_output(&self) -> Result<(), CoverageError>;
}

pub fn stub_compute_coverage() -> Box<MockComputeCoverage> {
    let mut mock = MockComputeCoverage::new();
    mock.expect_clean_coverage_output().returning(|| { Ok(()) });
    mock.expect_compute_coverage().returning(|_cancellation| { Ok(CoverageStatus::Disabled) });

    Box::new(mock)
}