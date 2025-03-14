use std::sync::mpsc::channel;
use crate::assert_matches;
use crate::configuration::TestRunnerImplementation;
use crate::test_execution::{ChangeEventHandler, TestRunProcessor};
use crate::test_run_model::TestRunState;
use crate::coverage::CoverageStatus;
use crate::change_events::{ChangeEvent, HandleChangeEvent};
use crate::test_execution::MockRunTests;
use crate::coverage::MockComputeCoverage;
use crate::test_execution::MockParseOutput;

#[test]
pub fn when_test_run_fails_error_is_reported() {  
    let mut run_tests = MockRunTests::new();

    run_tests.expect_run_tests()
        .returning(|_| { Err(std::io::Error::new(std::io::ErrorKind::NotFound, "Example error")) });

    let mut compute_coverage = MockComputeCoverage::new();
    compute_coverage.expect_clean_coverage_output().returning(|| Ok(()));
    compute_coverage.expect_compute_coverage().returning(|| Ok(CoverageStatus::Disabled));

    let mut parser = MockParseOutput::new();
    parser.expect_parse_line().returning(|_line| None);
    parser.expect_get_implementation().returning(|| TestRunnerImplementation::Cargo);

    let processor = TestRunProcessor::new(Box::new(run_tests), Box::new(parser));
    let (tests_sender, tests_receiver) = channel();
    let (coverage_sender, _coverage_receiver) = channel();
    let mut handler = ChangeEventHandler::new(processor, Box::new(compute_coverage), tests_sender, coverage_sender);

    handler.handle_event(ChangeEvent::File);

    let _start = tests_receiver.recv().unwrap();
    let error = tests_receiver.recv().unwrap().state;

    assert_matches!(error, TestRunState::Failed);
}
