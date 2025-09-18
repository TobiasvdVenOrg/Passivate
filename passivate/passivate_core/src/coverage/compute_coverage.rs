use passivate_delegation::Cancellation;

use super::{CoverageError, CoverageStatus};

#[mockall::automock]
pub trait ComputeCoverage
{
    fn compute_coverage(&self, cancellation: Cancellation) -> Result<CoverageStatus, CoverageError>;
    fn clean_coverage_output(&self) -> Result<(), CoverageError>;
}

pub fn stub() -> impl ComputeCoverage 
{
    let mut mock = MockComputeCoverage::new();
    mock.expect_clean_coverage_output().returning(|| Ok(()));
    mock.expect_compute_coverage().returning(|_cancellation| Ok(CoverageStatus::Disabled));

    mock
}
