use passivate_delegation::Cancellation;

use super::{CoverageError, CoverageStatus};

#[mockall::automock]
pub trait ComputeCoverage
{
    fn compute_coverage(&self, cancellation: Cancellation) -> Result<CoverageStatus, CoverageError>;
    fn clean_coverage_output(&self) -> Result<(), CoverageError>;
}

pub type BComputeCoverage = Box<dyn ComputeCoverage + Send>;
pub trait Stub<T>
{
    fn stub() -> T;   
}

impl Stub<BComputeCoverage> for BComputeCoverage
{
    fn stub() -> BComputeCoverage 
    {
        let mut mock = MockComputeCoverage::new();
        mock.expect_clean_coverage_output().returning(|| Ok(()));
        mock.expect_compute_coverage().returning(|_cancellation| Ok(CoverageStatus::Disabled));

        Box::new(mock)
    }
}
