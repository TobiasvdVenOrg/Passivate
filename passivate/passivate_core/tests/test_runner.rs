use std::path::Path;
use std::sync::mpsc::{channel, Sender};
use passivate_core::change_events::{ChangeEvent, HandleChangeEvent};
use passivate_core::test_execution::{TestRunner, TestsStatus};
use std::fs;

macro_rules! assert_matches {
    ($value:expr, $pattern:pat $( if $guard:expr )?) => {
        match &$value {
            $pattern $( if $guard )? => (),
            _ => panic!(
                "assertion failed: expected `{}` to match `{}`",
                stringify!($value),
                stringify!($pattern)
            ),
        }
    };
}

#[cfg(target_os = "windows")]
#[test]
pub fn change_event_causes_test_run_and_results() {
    let path = "../../test_data/sample_project";
    let (sender, receiver) = channel();

    mock_test_run(path, sender);

    let running = receiver.recv().unwrap();
    let completed = receiver.recv().unwrap();

    assert_matches!(running, TestsStatus::Running);
    assert_matches!(completed, TestsStatus::Completed(completed) if completed.tests.len() == 3);
}

#[cfg(target_os = "windows")]
#[test]
pub fn test_run_outputs_coverage_file_for_project() {
    let path = "../../test_data/sample_project";
    let passivate_path = "../../test_data/sample_project/.passivate";
    clean_passivate_dir(passivate_path);

    let (sender, _receiver) = channel();

    mock_test_run(path, sender);
    
    let expected_output_path = fs::canonicalize("../../test_data/sample_project/.passivate/coverage/lcov.info").unwrap();

    let file_data = fs::metadata(&expected_output_path);
    assert!(file_data.is_ok(), "Expected coverage output file did not exist: {}", expected_output_path.display());
}

#[cfg(target_os = "windows")]
#[test]
pub fn test_run_outputs_coverage_file_for_workspace() {
    let path = "../../test_data/sample_workspace";
    let passivate_path = "../../test_data/sample_workspace/.passivate";
    clean_passivate_dir(passivate_path);

    let (sender, _receiver) = channel();

    mock_test_run(path, sender);
    
    let expected_output_path = fs::canonicalize("../../test_data/sample_workspace/.passivate/coverage/lcov.info").unwrap();

    let file_data = fs::metadata(&expected_output_path);
    assert!(file_data.is_ok(), "Expected coverage output file did not exist: {}", expected_output_path.display());
}

fn mock_test_run(path: &str, sender: Sender<TestsStatus>) {
    let mut test_runner = TestRunner::new(Path::new(path), sender);

    let mock_event = ChangeEvent { };
    test_runner.handle_event(mock_event);
}

fn clean_passivate_dir(path: &str) {
    if fs::exists(path).expect("Failed to even check if /.passivate directory exists!") {
        fs::remove_dir_all(path).expect("Failed to remove /.passivate directory before test run!");
    }
}