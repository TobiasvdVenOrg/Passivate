use std::io::Error as IoError;
use std::sync::mpsc::channel;
use passivate_core::assert_matches;
use passivate_core::test_execution::TestRun;
mod helpers;
use helpers::*;
use rstest::*;
use stdext::function_name;

// TODO: Rename this file without 'cargo'

#[cfg(target_os = "windows")]
#[rstest]
#[case::cargo(cargo_builder())]
#[case::nextest(nextest_builder())]
pub fn tests_status_is_completed_after_run_with_no_tests_found(#[case] mut builder: TestRunnerBuilder) -> Result<(), IoError> {
    let (sender, receiver) = channel();
    let mut runner = builder
        .receive_tests_status(sender)
        .with_workspace("no_tests_project")
        .with_output(function_name!())
        .build();

    test_run(&mut runner)?;

    let _running = receiver.try_recv().unwrap();
    let completed = receiver.try_recv().unwrap();

    let completed = assert_matches!(completed, TestRun::Active);
    assert_eq!(0, completed.tests.len());

    Ok(())
}

#[cfg(target_os = "windows")]
#[rstest]
#[case::cargo(cargo_builder())]
#[case::nextest(nextest_builder())]
pub fn change_event_causes_test_run_and_results(#[case] mut builder: TestRunnerBuilder) -> Result<(), IoError> {
    let (sender, receiver) = channel();
    let mut runner = builder
        .receive_tests_status(sender)
        .with_workspace("simple_project")
        .with_output(function_name!())
        .build();

    test_run(&mut runner)?;

    let _running = receiver.try_recv().unwrap();
    let _test1 = receiver.try_recv().unwrap();
    let _test2 = receiver.try_recv().unwrap();
    let test3 = receiver.try_recv().unwrap();

    let completed = assert_matches!(test3, TestRun::Active);
    assert_eq!(3, completed.tests.len());

    Ok(())
}
