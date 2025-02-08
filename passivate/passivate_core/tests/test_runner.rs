#![feature(assert_matches)]

use std::assert_matches::assert_matches;
use std::fs;
use std::path::Path;
use std::sync::mpsc::{channel, Sender};
use passivate_core::change_events::{ChangeEvent, HandleChangeEvent};
use passivate_core::test_execution::{TestRunner, TestsStatus};

#[test]
pub fn change_event_causes_test_run_and_results() {
    let (sender, receiver) = channel();

    mock_test_run(sender);

    let running = receiver.recv().unwrap();
    let completed = receiver.recv().unwrap();

    assert_matches!(running, TestsStatus::Running);
    assert_matches!(completed, TestsStatus::Completed(completed) if completed.tests.len() == 3);
}

#[test]
pub fn test_run_outputs_coverage_file() {

    let (sender, receiver) = channel();

    mock_test_run(sender);

    let expected_output_path = "../sample_project/.passivate/coverage/lcov";

    assert!(fs::metadata(expected_output_path).is_ok(), "Expected coverage output file did not exist!");
}

fn build_test_runner(sender: Sender<TestsStatus>) -> TestRunner {
    let path = Path::new("../sample_project");
    TestRunner::new(path, sender)
}

fn mock_test_run(sender: Sender<TestsStatus>) {
    let mut test_runner = build_test_runner(sender);

    let mock_event = ChangeEvent { };
    test_runner.handle_event(mock_event);
}