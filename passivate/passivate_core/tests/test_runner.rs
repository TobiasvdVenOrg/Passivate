mod helpers;

use std::io::Error as IoError;
use std::path::Path;
use std::sync::mpsc::{channel, Sender};
use passivate_core::change_events::{ChangeEvent, HandleChangeEvent};
use passivate_core::passivate_cargo::CargoTest;
use passivate_core::passivate_grcov::Grcov;
use passivate_core::test_execution::{TestRunner, TestsStatus};
use std::fs;

#[cfg(target_os = "windows")]
#[test]
pub fn change_event_causes_test_run_and_results() {
    let path = Path::new("../../test_data/change_event_causes_test_run_and_results");
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
    let path = Path::new("../../test_data/test_run_outputs_coverage_file_for_project");
    let passivate_path = Path::new("../../test_data/test_run_outputs_coverage_file_for_project/.passivate");    
    
    clean_passivate_dir(passivate_path);

    let (sender, _receiver) = channel();

    mock_test_run(path, sender);
    
    let expected_output_path = fs::canonicalize("../../test_data/test_run_outputs_coverage_file_for_project/.passivate/coverage/lcov.info").unwrap();

    let file_data = fs::metadata(&expected_output_path);
    assert!(file_data.is_ok(), "Expected coverage output file did not exist: {}", expected_output_path.display());
}

#[cfg(target_os = "windows")]
#[test]
pub fn test_run_outputs_coverage_file_for_workspace() {
    let path = Path::new("../../test_data/test_run_outputs_coverage_file_for_workspace");
    let passivate_path = Path::new("../../test_data/test_run_outputs_coverage_file_for_workspace/.passivate");
    clean_passivate_dir(passivate_path);

    let (sender, _receiver) = channel();

    mock_test_run(path, sender);
    
    let expected_output_path = fs::canonicalize("../../test_data/test_run_outputs_coverage_file_for_workspace/.passivate/coverage/lcov.info").unwrap();

    let file_data = fs::metadata(&expected_output_path);
    assert!(file_data.is_ok(), "Expected coverage output file did not exist: {}", expected_output_path.display());
}

#[cfg(target_os = "windows")]
#[test]
pub fn repeat_test_runs_do_not_accumulate_profraw_files() -> Result<(), IoError> {
    let path = Path::new("../../test_data/repeat_test_runs_do_not_accumulate_profraw_files");
    let passivate_path = Path::new("../../test_data/repeat_test_runs_do_not_accumulate_profraw_files/.passivate");
    clean_passivate_dir(passivate_path);

    let (sender, _receiver) = channel();
    let mut test_runner = new_test_runner(path, sender);

    let mock_event = ChangeEvent { };
    test_runner.handle_event(mock_event);
    
    let profraw_directory = passivate_path.join("coverage");
    let first_run = get_profraw_count(profraw_directory.as_path())?;

    let mock_event = ChangeEvent { };
    test_runner.handle_event(mock_event);

    let second_run = get_profraw_count(profraw_directory.as_path())?; 

    assert_ne!(0, second_run);
    assert_eq!(first_run, second_run);
    Ok(())
}

#[cfg(target_os = "windows")]
#[test]
// Temporary deletion of the lcov.info file before re-creation can cause coverage systems relying on it (like Coverage Gutters in VSCode)
// to briefly error due to "not finding the file" until a new one is created
pub fn repeat_test_runs_do_not_delete_lcov_file() -> Result<(), IoError> {
    let path = Path::new("../../test_data/repeat_test_runs_do_not_delete_lcov_file");
    let passivate_path = Path::new("../../test_data/repeat_test_runs_do_not_delete_lcov_file/.passivate");
    clean_passivate_dir(passivate_path);

    let (sender, _receiver) = channel();
    let mut test_runner = new_test_runner(path, sender);

    let mock_event = ChangeEvent { };
    test_runner.handle_event(mock_event);
    
    let lcov_path = passivate_path.join("coverage").join("lcov.info");
    let first_run_metadata = fs::metadata(&lcov_path)?;

    let mock_event = ChangeEvent { };
    test_runner.handle_event(mock_event);

    let second_run_metadata = fs::metadata(&lcov_path)?;
    
    assert_eq!(first_run_metadata.created()?, second_run_metadata.created()?);
    Ok(())
}

#[test]
pub fn grcov_not_installed_reports_bla() -> Result<(), IoError> {
    Ok(()) 
}

fn new_test_runner(path: &Path, sender: Sender<TestsStatus>) -> TestRunner {
    TestRunner::new(path, Box::new(CargoTest { }), Box::new(Grcov { }), sender)
}

fn mock_test_run(path: &Path, sender: Sender<TestsStatus>) {
    let mut test_runner = new_test_runner(path, sender);

    let mock_event = ChangeEvent { };
    test_runner.handle_event(mock_event);
}

fn clean_passivate_dir(path: &Path) {
    if fs::exists(path).expect("Failed to even check if /.passivate directory exists!") {
        if let Err(error) = fs::remove_dir_all(path) {
            println!("Failed to remove /.passivate directory before test run! {}", error);
        }
    }
}

fn get_profraw_count(passivate_path: &Path) -> Result<i32, IoError> {
    
    let mut count = 0;

    for profraw in fs::read_dir(passivate_path)? {
        if let Some(extension) = profraw?.path().extension() {
            if extension == "profraw" {
                count += 1;
            }
        }
    }

    Ok(count)
}