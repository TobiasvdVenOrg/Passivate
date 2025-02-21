use crate::change_events::{ChangeEvent, HandleChangeEvent};
use crate::coverage::{ComputeCoverage, CoverageStatus};
use crate::dispatching::Dispatch;
use crate::passivate_cargo::*;
use crate::test_execution::TestsStatus;
use super::{RunTests, RunTestsErrorStatus, TestRunnerStatus, TestRunnerStatusDispatch};

pub struct _TestRunner<T: Dispatch<TestRunnerStatus>> {
    runner: Box<dyn RunTests>,
    coverage: Box<dyn ComputeCoverage>,
    status_handler: T
}

pub type TestRunner = _TestRunner<TestRunnerStatusDispatch>;

impl<T: Dispatch<TestRunnerStatus>> _TestRunner<T> {
    pub fn new(
        runner: Box<dyn RunTests>,
        coverage: Box<dyn ComputeCoverage>, 
        status_handler: T) -> Self {
            Self {
            runner, 
            coverage, 
            status_handler
        }
    }
}

impl<T: Dispatch<TestRunnerStatus>> HandleChangeEvent for _TestRunner<T> {
    fn handle_event(&mut self, _event: ChangeEvent) {
        let _ = self.status_handler.dispatch(TestsStatus::Running.into());

        let _ = self.coverage.clean_coverage_output();

        let test_output = self.runner.run_tests();

        match test_output {
            Ok(test_output) => {
                let tests_status = parse_status(&test_output);
                let _ = self.status_handler.dispatch(tests_status.into());

                let coverage_status = self.coverage.compute_coverage();

                let _ = match coverage_status {
                    Ok(coverage_status) => self.status_handler.dispatch(coverage_status.into()),
                    Err(coverage_error) => self.status_handler.dispatch(CoverageStatus::Error(coverage_error).into())
                };
            },
            Err(test_error) => {
                let error_status = RunTestsErrorStatus { inner_error_display: test_error.to_string() };
                let _  = self.status_handler.dispatch(TestsStatus::RunTestsError(error_status).into());
            }
        };
    }
}
