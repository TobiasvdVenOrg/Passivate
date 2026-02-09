use crate::coverage_errors::CoverageError;
use crate::coverage_status::CoverageStatus;

#[mockall::automock]
pub trait ComputeCoverage
{
    fn compute_coverage(&self) -> Result<CoverageStatus, CoverageError>;
    fn clean_coverage_output(&self) -> Result<(), CoverageError>;
}

pub fn stub() -> impl ComputeCoverage
{
    let mut mock = MockComputeCoverage::new();
    mock.expect_clean_coverage_output().returning(|| Ok(()));
    mock.expect_compute_coverage().returning(|| Ok(CoverageStatus::Disabled));

    mock
}
