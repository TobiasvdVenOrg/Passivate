use std::sync::mpsc::channel;
use crate::{assert_matches, change_events::{ChangeEvent, HandleChangeEvent}, coverage::{CoverageStatus, MockComputeCoverage}, test_execution::{MockRunTests, RunTestsError, TestRunner, TestsStatus}};



#[test]
pub fn when_test_run_fails_error_is_reported() {   
    let mut run_tests = MockRunTests::new();
    run_tests.expect_run_tests().returning(|| {
        let error = String::from_utf8(vec!(0, 159)).err().unwrap();
        Err(RunTestsError::Output(error))
    });

    let mut compute_coverage = MockComputeCoverage::new();
    compute_coverage.expect_clean_coverage_output().returning(|| Ok(()));
    compute_coverage.expect_compute_coverage().returning(|| Ok(CoverageStatus::Disabled));

    let (tests_handler, receiver) = channel();
    let mut test_runner = TestRunner::new(Box::new(run_tests), Box::new(compute_coverage), tests_handler);

    test_runner.handle_event(ChangeEvent { });

    let _running = receiver.recv().unwrap();
    let error = receiver.recv().unwrap();

    assert_matches!(error, TestsStatus::RunTestsError);
}