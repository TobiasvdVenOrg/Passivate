use std::sync::mpsc::channel;
use crate::{assert_matches, change_events::{ChangeEvent, HandleChangeEvent}, coverage::{CoverageError, CoverageStatus, MockComputeCoverage}, test_execution::{MockRunTests, RunTestsError, TestRunner, TestRunnerStatusDispatch, TestsStatus}};

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

    let (tests_sender, tests_receiver) = channel();
    let (coverage_sender, _coverage_receiver) = channel();
    let dispatch = TestRunnerStatusDispatch::new(tests_sender, coverage_sender);
    let mut test_runner = TestRunner::new(Box::new(run_tests), Box::new(compute_coverage), dispatch);

    test_runner.handle_event(ChangeEvent::File);

    let _running = tests_receiver.recv().unwrap();
    let error = tests_receiver.recv().unwrap();

    assert_matches!(error, TestsStatus::RunTestsError);
}

#[test]
pub fn when_grcov_is_not_installed_error_is_reported() {
    let mut run_tests = MockRunTests::new();
    run_tests.expect_run_tests().returning(|| {
        Ok("".to_string())
    });

    let mut compute_coverage = MockComputeCoverage::new();
    compute_coverage.expect_clean_coverage_output().returning(|| Ok(()));
    compute_coverage.expect_compute_coverage().returning(|| {
        Err(CoverageError::GrcovNotInstalled(std::io::ErrorKind::NotFound))
    });

    let (tests_sender, _tests_receiver) = channel();
    let (coverage_sender, coverage_receiver) = channel();
    let dispatch = TestRunnerStatusDispatch::new(tests_sender, coverage_sender);
    let mut test_runner = TestRunner::new(Box::new(run_tests), Box::new(compute_coverage), dispatch);

    test_runner.handle_event(ChangeEvent::File);

    let error = coverage_receiver.recv().unwrap();

    assert_matches!(error, CoverageStatus::Error);
}