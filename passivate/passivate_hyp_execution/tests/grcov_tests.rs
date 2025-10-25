#![cfg(target_os = "windows")]

use std::fs;
use std::io::Error as IoError;

use camino::{Utf8Path, Utf8PathBuf};
use passivate_coverage::{compute_coverage::ComputeCoverage, coverage_errors::{CoverageError, NoProfrawFilesKind}, coverage_status::CoverageStatus, grcov::get_profraw_count};
use passivate_delegation::{Cancellation, Tx};
use passivate_hyp_execution::test_helpers::test_run_setup::TestRunSetup;
use passivate_hyp_names::test_name;
use passivate_testing::path_resolution::test_output_path;
use pretty_assertions::assert_eq;

use passivate_hyp_model::change_event::ChangeEvent;

#[test]
pub fn test_run_sends_coverage_result() -> Result<(), IoError>
{
    let (coverage_tx, coverage_rx) = Tx::new();
    let setup = TestRunSetup::builder(test_name!(), "simple_project")
        .coverage_enabled(true)
        .coverage_sender(coverage_tx)
        .build();

    setup.clean_output().build_test_run_handler().handle(ChangeEvent::DefaultRun, Cancellation::default());

    let result = coverage_rx.drain().last().unwrap().clone();

    match result
    {
        CoverageStatus::Disabled => panic!(),
        CoverageStatus::Preparing => panic!(),
        CoverageStatus::Running => panic!(),
        CoverageStatus::Done(covdir_json) =>
        {
            assert_eq!(100.0, covdir_json.coverage_percent);
        }
        CoverageStatus::Error(_) => panic!()
    };

    Ok(())
}

#[test]
pub fn test_run_outputs_coverage_file_for_project() -> Result<(), IoError>
{
    let setup = TestRunSetup::builder(test_name!(), "simple_project").coverage_enabled(true).build();

    let output_path = setup.get_output_path();

    setup.clean_output().build_test_run_handler().handle(ChangeEvent::DefaultRun, Cancellation::default());

    let file_data = expected_lcov_metadata(&output_path);
    assert!(file_data.is_ok(), "Expected coverage output file did not exist: {:?}", file_data);

    Ok(())
}

#[test]
pub fn test_run_outputs_coverage_file_for_workspace() -> Result<(), IoError>
{
    let setup = TestRunSetup::builder(test_name!(), "simple_workspace").coverage_enabled(true).build();

    let output_path = setup.get_output_path();

    setup.clean_output().build_test_run_handler().handle(ChangeEvent::DefaultRun, Cancellation::default());

    let file_data = expected_lcov_metadata(&output_path);
    assert!(file_data.is_ok(), "Expected coverage output file did not exist: {:?}", file_data);

    Ok(())
}

#[test]
pub fn repeat_test_runs_do_not_accumulate_profraw_files() -> Result<(), IoError>
{
    let setup = TestRunSetup::builder(test_name!(), "simple_project").coverage_enabled(true).build();

    let coverage_path = setup.get_coverage_path();

    let mut handler = setup.clean_output().build_test_run_handler();

    handler.handle(ChangeEvent::DefaultRun, Cancellation::default());

    let first_run = get_profraw_count(&coverage_path)?;

    handler.handle(ChangeEvent::DefaultRun, Cancellation::default());

    let second_run = get_profraw_count(&coverage_path)?;

    assert_ne!(0, second_run);
    assert_eq!(first_run, second_run);
    Ok(())
}

#[test]
// Temporary deletion of the lcov file before re-creation can cause coverage systems relying on it (like Coverage Gutters in VSCode)
// to briefly error due to "not finding the file" until a new one is created
pub fn repeat_test_runs_do_not_delete_lcov_file() -> Result<(), IoError>
{
    let setup = TestRunSetup::builder(test_name!(), "simple_project").coverage_enabled(true).build();

    let output_path = setup.get_output_path();

    let mut handler = setup.clean_output().build_test_run_handler();

    handler.handle(ChangeEvent::DefaultRun, Cancellation::default());

    let first_run_metadata = expected_lcov_metadata(&output_path)?;

    handler.handle(ChangeEvent::DefaultRun, Cancellation::default());

    let second_run_metadata = expected_lcov_metadata(&output_path)?;

    assert_eq!(first_run_metadata.created()?, second_run_metadata.created()?);
    Ok(())
}

#[test]
pub fn error_when_coverage_is_computed_with_no_profraw_files_present() -> Result<(), IoError>
{
    let setup = TestRunSetup::builder(test_name!(), "simple_project").coverage_enabled(true).build();

    let coverage_path = setup.get_coverage_path();

    let setup = setup.clean_output();

    fs::create_dir_all(&coverage_path)?;

    let grcov = setup.build_grcov();

    let result = grcov.compute_coverage(Cancellation::default());

    assert!(result.is_err_and(|e| {
        match e
        {
            CoverageError::NoProfrawFiles(details) =>
            {
                assert_eq!(coverage_path, details.expected_path);
                assert_eq!(NoProfrawFilesKind::NoProfrawFilesExist, details.kind);

                true
            }
            _ => false
        }
    }));

    Ok(())
}

#[test]
pub fn error_when_coverage_is_computed_and_profraw_output_directory_does_not_exist() -> Result<(), IoError>
{
    let setup = TestRunSetup::builder(test_name!(), "simple_project").coverage_enabled(true).build();

    let coverage_path = setup.get_coverage_path();

    let grcov = setup.clean_output().build_grcov();

    let result = grcov.compute_coverage(Cancellation::default());

    assert!(result.is_err_and(|e| {
        match e
        {
            CoverageError::NoProfrawFiles(details) =>
            {
                assert_eq!(coverage_path, details.expected_path);

                match details.kind
                {
                    NoProfrawFilesKind::Io(error_kind) =>
                    {
                        assert_eq!(std::io::ErrorKind::NotFound, error_kind)
                    }
                    _ => panic!()
                };

                true
            }
            _ => false
        }
    }));

    Ok(())
}

#[test]
pub fn no_coverage_related_files_are_generated_when_coverage_is_disabled() -> Result<(), IoError>
{
    let setup = TestRunSetup::builder(test_name!(), "simple_project").coverage_enabled(false).build();

    let coverage_path = setup.get_coverage_path();
    let output_path = setup.get_output_path();

    setup.clean_output().build_test_run_handler().handle(ChangeEvent::DefaultRun, Cancellation::default());

    let profraw_count = get_profraw_count(&coverage_path)?;
    assert_eq!(0, profraw_count);

    let exists = fs::exists(expected_lcov_path(&output_path))?;
    assert!(!exists, "lcov file existed unexpectedly");

    Ok(())
}

fn expected_lcov_path(test_name: &Utf8Path) -> Utf8PathBuf
{
    test_output_path().join(test_name).join(".passivate/coverage/lcov")
}

fn expected_lcov_metadata(test_name: &Utf8Path) -> Result<fs::Metadata, IoError>
{
    let path = expected_lcov_path(test_name);
    fs::metadata(path)
}
