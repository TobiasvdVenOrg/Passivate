use std::io::Error as IoError;
use std::path::Path;
use std::sync::mpsc::channel;
use passivate_core::change_events::{ChangeEvent, HandleChangeEvent};
use passivate_core::passivate_cargo::CargoTest;
use passivate_core::passivate_grcov::Grcov;
use passivate_core::test_execution::{TestRunner, TestRunnerStatusDispatch};
use std::fs;

#[cfg(target_os = "windows")]
#[test]
pub fn test_run_outputs_coverage_file_for_project() -> Result<(), IoError> {
    let path = Path::new("../../test_data/test_run_outputs_coverage_file_for_project");  
    clean_passivate_dir(path)?;

    new_test_run(path)?;
    
    let expected_output_path = fs::canonicalize("../../test_data/test_run_outputs_coverage_file_for_project/.passivate/coverage/lcov.info")?;

    let file_data = fs::metadata(&expected_output_path);
    assert!(file_data.is_ok(), "Expected coverage output file did not exist: {}", expected_output_path.display());

    Ok(())
}

#[cfg(target_os = "windows")]
#[test]
pub fn test_run_outputs_coverage_file_for_workspace() -> Result<(), IoError> {
    let path = Path::new("../../test_data/test_run_outputs_coverage_file_for_workspace");
    clean_passivate_dir(path)?;

    new_test_run(path)?;
    
    let expected_output_path = fs::canonicalize("../../test_data/test_run_outputs_coverage_file_for_workspace/.passivate/coverage/lcov.info")?;

    let file_data = fs::metadata(&expected_output_path);
    assert!(file_data.is_ok(), "Expected coverage output file did not exist: {}", expected_output_path.display());

    Ok(())
}

#[cfg(target_os = "windows")]
#[test]
pub fn repeat_test_runs_do_not_accumulate_profraw_files() -> Result<(), IoError> {
    let path = Path::new("../../test_data/repeat_test_runs_do_not_accumulate_profraw_files");
    clean_passivate_dir(path)?;

    let mut test_runner = new_test_runner(path)?;

    test_run(&mut test_runner)?;
    
    let first_run = get_profraw_count(path)?;

    test_run(&mut test_runner)?;

    let second_run = get_profraw_count(path)?; 

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
    clean_passivate_dir(path)?;

    let mut test_runner = new_test_runner(path)?;

    test_run(&mut test_runner)?;
    
    let lcov_path = path.join(".passivate").join("coverage").join("lcov.info");
    let first_run_metadata = fs::metadata(&lcov_path)?;

    test_run(&mut test_runner)?;

    let second_run_metadata = fs::metadata(&lcov_path)?;
    
    assert_eq!(first_run_metadata.created()?, second_run_metadata.created()?);
    Ok(())
}

fn new_test_runner(path: &Path) -> Result<TestRunner, IoError> {
    let passivate_path = path.join(".passivate");
    let binary_path = Path::new("./target/x86_64-pc-windows-msvc/debug/");
    let coverage_path = path.join(".passivate").join("coverage");
    fs::create_dir_all(&coverage_path)?;
    let absolute_coverage_path = fs::canonicalize(passivate_path.join("coverage"))?; 
    let grcov = Grcov::new(path, &coverage_path, binary_path);
    let cargo_test = CargoTest::new(path, &absolute_coverage_path);
    let (tests_sender, _tests_receiver) = channel();
    let (coverage_sender, _coverage_receiver) = channel();
    let dispatch = TestRunnerStatusDispatch::new(tests_sender, coverage_sender);
    Ok(TestRunner::new(Box::new(cargo_test), Box::new(grcov), dispatch))
}

fn new_test_run(path: &Path) -> Result<(), IoError> {
    let mut test_runner = new_test_runner(path)?;

    test_run(&mut test_runner)
}

fn test_run(test_runner: &mut TestRunner) -> Result<(), IoError> {
    let mock_event = ChangeEvent::File;
    test_runner.handle_event(mock_event);

    Ok(())
}

fn clean_passivate_dir(path: &Path) -> Result<(), IoError> {
    let passivate_path = path.join(".passivate");
    if fs::exists(&passivate_path)? {
        fs::remove_dir_all(&passivate_path)?
    }

    Ok(())
}

fn get_profraw_count(path: &Path) -> Result<i32, IoError> {
    let coverage_path = path.join(".passivate").join("coverage");
    let mut count = 0;

    for profraw in fs::read_dir(coverage_path)? {
        if let Some(extension) = profraw?.path().extension() {
            if extension == "profraw" {
                count += 1;
            }
        }
    }

    Ok(count)
}