use std::sync::mpsc::Sender;
use crate::{coverage::CoverageStatus, dispatching::{Dispatch, DispatchError}};
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
