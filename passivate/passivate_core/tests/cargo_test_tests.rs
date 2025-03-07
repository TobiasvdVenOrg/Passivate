use std::io::Error as IoError;
use std::sync::mpsc::channel;
use passivate_core::assert_matches;
use passivate_core::test_execution::TestsStatus;
mod helpers;
use helpers::*;
use rstest::*;

// TODO: Rename this file without 'cargo'

#[cfg(target_os = "windows")]
#[rstest]
#[case::cargo(cargo_builder())]
#[case::nextest(nextest_builder())]
pub fn change_event_causes_test_run_and_results(#[case] mut builder: TestRunnerBuilder) -> Result<(), IoError> {
    let (sender, receiver) = channel();
    let mut runner = builder
        .receive_tests_status(sender)
        .with_workspace("simple_project")
        .with_output("change_event_causes_test_run_and_results")
        .build()?;

    test_run(&mut runner)?;

    let _running = receiver.try_recv().unwrap();
    let _test1 = receiver.try_recv().unwrap();
    let _test2 = receiver.try_recv().unwrap();
    let test3 = receiver.try_recv().unwrap();

    let completed = assert_matches!(test3, TestsStatus::Completed);
    assert_eq!(3, completed.tests.len());

    Ok(())
}
