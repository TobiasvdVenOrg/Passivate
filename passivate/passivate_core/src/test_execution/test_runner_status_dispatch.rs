
use std::sync::mpsc::Sender;

use crate::{coverage::CoverageStatus, dispatching::{Dispatch, DispatchError}};

use super::{TestRunnerStatus, TestsStatus};

pub struct _TestRunnerStatusDispatch<TTestsDispatch: Dispatch<TestsStatus>, TCoverageDispatch: Dispatch<CoverageStatus>> {
    tests_dispatch: TTestsDispatch,
    coverage_dispatch: TCoverageDispatch
}

pub type TestRunnerStatusDispatch = _TestRunnerStatusDispatch<Sender<TestsStatus>, Sender<CoverageStatus>>;

impl<TTestsDispatch: Dispatch<TestsStatus>, TCoverageDispatch: Dispatch<CoverageStatus>> _TestRunnerStatusDispatch<TTestsDispatch, TCoverageDispatch> {
    pub fn new(tests_dispatch: TTestsDispatch, coverage_dispatch: TCoverageDispatch) -> Self {
        Self { tests_dispatch, coverage_dispatch }
    }
}

impl Dispatch<TestRunnerStatus> for TestRunnerStatusDispatch {
    fn dispatch(&self, payload: TestRunnerStatus) -> Result<(), DispatchError> {
        match payload {
            TestRunnerStatus::Tests(tests_status) => self.tests_dispatch.dispatch(tests_status),
            TestRunnerStatus::Coverage(coverage_status) => self.coverage_dispatch.dispatch(coverage_status)
        }
    }
}