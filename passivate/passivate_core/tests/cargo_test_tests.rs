use std::fs;
use std::io::Error as IoError;
use std::path::Path;
use std::sync::mpsc::{channel, Sender};
use passivate_core::assert_matches;
use passivate_core::change_events::{ChangeEvent, HandleChangeEvent};
use passivate_core::passivate_cargo::CargoTest;
use passivate_core::passivate_grcov::Grcov;
use passivate_core::passivate_nextest::Nextest;
use passivate_core::test_execution::{TestRunner, TestsStatus};

#[cfg(target_os = "windows")]
#[test]
pub fn change_event_causes_test_run_and_results() -> Result<(), IoError> {
    let path = Path::new("../../test_data/change_event_causes_test_run_and_results");
    let (sender, receiver) = channel();

    new_test_run(path, sender)?;

    let _running = receiver.recv().unwrap();
    let _test1 = receiver.recv().unwrap();
    let _test2 = receiver.recv().unwrap();
    let test3 = receiver.recv().unwrap();

    let completed = assert_matches!(test3, TestsStatus::Completed);
    assert_eq!(3, completed.tests.len());

    Ok(())
}

fn new_test_runner(path: &Path, tests_status: Sender<TestsStatus>) -> Result<TestRunner, IoError> {
    let passivate_path = path.join(".passivate");
    let binary_path = Path::new("./target/x86_64-pc-windows-msvc/debug/");
    let coverage_path = path.join(".passivate").join("coverage");
    fs::create_dir_all(&coverage_path)?;
    let absolute_coverage_path = fs::canonicalize(passivate_path.join("coverage"))?; 
    let grcov = Grcov::new(path, &coverage_path, binary_path);
    let cargo_test = CargoTest::new(path, &absolute_coverage_path);
    let nextest = Nextest::new(path, &absolute_coverage_path);
    let (coverage_sender, _coverage_receiver) = channel();
    Ok(TestRunner::new(Box::new(nextest), Box::new(grcov), tests_status, coverage_sender))
}

fn new_test_run(path: &Path, tests_status: Sender<TestsStatus>) -> Result<(), IoError> {
    let mut test_runner = new_test_runner(path, tests_status)?;

    test_run(&mut test_runner)
}

fn test_run(test_runner: &mut TestRunner) -> Result<(), IoError> {

    let mock_event = ChangeEvent::File;
    test_runner.handle_event(mock_event);

    Ok(())
}