#![cfg(test)]

use std::sync::mpsc::channel;
use crate::assert_matches;
use crate::test_execution::{MockRunTests, RunTestsError, TestRunner, TestsStatus};
use crate::coverage::{CoverageStatus, MockComputeCoverage};
use crate::change_events::{ChangeEvent, HandleChangeEvent};

#[test]
pub fn when_test_run_fails_error_is_reported() {   
    let mut run_tests = MockRunTests::new();
    run_tests.expect_run_tests().returning(|_sender| {
        let error = String::from_utf8(vec!(0, 159)).err().unwrap();
        Err(RunTestsError::InvalidOutput(error))
    });

    let mut compute_coverage = MockComputeCoverage::new();
    compute_coverage.expect_clean_coverage_output().returning(|| Ok(()));
    compute_coverage.expect_compute_coverage().returning(|| Ok(CoverageStatus::Disabled));

    let (tests_sender, tests_receiver) = channel();
    let (coverage_sender, _coverage_receiver) = channel();
    let mut test_runner = TestRunner::new(Box::new(run_tests), Box::new(compute_coverage), tests_sender, coverage_sender);

    test_runner.handle_event(ChangeEvent::File);

    let error = tests_receiver.recv().unwrap();

    assert_matches!(error, TestsStatus::RunTestsError);
}
