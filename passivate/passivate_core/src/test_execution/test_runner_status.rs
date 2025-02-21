use crate::coverage::CoverageStatus;
use super::TestsStatus;

pub enum TestRunnerStatus {
    Tests(TestsStatus),
    Coverage(CoverageStatus)
}

impl From<TestsStatus> for TestRunnerStatus {
    fn from(value: TestsStatus) -> Self {
        TestRunnerStatus::Tests(value)
    }
}

impl From<CoverageStatus> for TestRunnerStatus {
    fn from(value: CoverageStatus) -> Self {
        TestRunnerStatus::Coverage(value)
    }
}
