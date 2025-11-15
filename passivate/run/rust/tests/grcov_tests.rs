#![cfg(target_os = "windows")]

#[macro_use] extern crate assert_matches;

mod helpers;

use std::fs;
use std::io::Error as IoError;

use camino::{Utf8Path, Utf8PathBuf};
use passivate_coverage::compute_coverage::ComputeCoverage;
use passivate_coverage::coverage_errors::{CoverageError, NoProfrawFilesKind};
use passivate_coverage::coverage_status::CoverageStatus;
use passivate_coverage::grcov::get_profraw_count;
use passivate_delegation::{Cancellation, Tx};
use passivate_model_core::hyp_run_trigger::HypRunTrigger;
use passivate_hyp_names::test_name;
use passivate_testing::path_resolution::test_output_path;
use passivate_testing::spy_log::SpyLog;
use passivate_testing::test_data_setup::TestDataSetup;
use pretty_assertions::assert_eq;

#[test]
pub fn test_run_sends_coverage_result() -> Result<(), IoError>
{
    SpyLog::set();

    let (coverage_tx, coverage_rx) = Tx::new();
    let setup = TestDataSetup::builder(test_name!(), "simple_project")
        .build()
        .clean_output();

    helpers::test_hyp_run_handler(&setup)
        .coverage_enabled(true)
        .coverage_tx(coverage_tx)
        .call()
        .handle(HypRunTrigger::DefaultRun, Cancellation::default());

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
    let setup = TestDataSetup::builder(test_name!(), "simple_project")
        .build()
        .clean_output();

    helpers::test_hyp_run_handler(&setup)
        .coverage_enabled(true)
        .call().handle(HypRunTrigger::DefaultRun, Cancellation::default());

    let file_data = expected_lcov_metadata(&setup.output_path());
    assert!(
        file_data.is_ok(),
        "Expected coverage output file did not exist: {:?}",
        file_data
    );

    Ok(())
}

#[test]
pub fn test_run_outputs_coverage_file_for_workspace() -> Result<(), IoError>
{
    let setup = TestDataSetup::builder(test_name!(), "simple_workspace")
        .build()
        .clean_output();

    helpers::test_hyp_run_handler(&setup)
        .coverage_enabled(true)
        .call()
        .handle(HypRunTrigger::DefaultRun, Cancellation::default());

    let file_data = expected_lcov_metadata(&setup.output_path());

    assert_matches!(file_data, Ok(_));

    Ok(())
}

#[test]
pub fn repeat_test_runs_do_not_accumulate_profraw_files() -> Result<(), IoError>
{
    let setup = TestDataSetup::builder(test_name!(), "simple_project")
        .build()
        .clean_output();

    let mut handler = helpers::test_hyp_run_handler(&setup)
        .coverage_enabled(true)
        .call();

    handler.handle(HypRunTrigger::DefaultRun, Cancellation::default());

    let first_run = get_profraw_count(&setup.coverage_path())?;

    handler.handle(HypRunTrigger::DefaultRun, Cancellation::default());

    let second_run = get_profraw_count(&setup.coverage_path())?;

    assert_ne!(0, second_run);
    assert_eq!(first_run, second_run);
    Ok(())
}

#[test]
// Temporary deletion of the lcov file before re-creation can cause coverage systems relying on it (like Coverage Gutters in VSCode)
// to briefly error due to "not finding the file" until a new one is created
pub fn repeat_test_runs_do_not_delete_lcov_file() -> Result<(), IoError>
{
    let setup = TestDataSetup::builder(test_name!(), "simple_project")
        .build()
        .clean_output();

    let mut handler = helpers::test_hyp_run_handler(&setup)
        .coverage_enabled(true)
        .call();

    handler.handle(HypRunTrigger::DefaultRun, Cancellation::default());

    let first_run_metadata = expected_lcov_metadata(&setup.output_path())?;

    handler.handle(HypRunTrigger::DefaultRun, Cancellation::default());

    let second_run_metadata = expected_lcov_metadata(&setup.output_path())?;

    assert_eq!(first_run_metadata.created()?, second_run_metadata.created()?);
    Ok(())
}

#[test]
pub fn error_when_coverage_is_computed_with_no_profraw_files_present() -> Result<(), IoError>
{
    let setup = TestDataSetup::builder(test_name!(), "simple_project")
        .build()
        .clean_output();

    fs::create_dir_all(setup.coverage_path())?;

    let grcov = helpers::test_grcov(&setup).call();

    let result = grcov.compute_coverage(Cancellation::default());

    assert_matches!(result, Err(CoverageError::NoProfrawFiles(details)) =>
    {
        assert_eq!(details.expected_path, setup.coverage_path());
        assert_matches!(details.kind, NoProfrawFilesKind::NoProfrawFilesExist);
    });

    Ok(())
}

#[test]
pub fn error_when_coverage_is_computed_and_profraw_output_directory_does_not_exist() -> Result<(), IoError>
{
    let setup = TestDataSetup::builder(test_name!(), "simple_project")
        .build()
        .clean_output();

    let grcov = helpers::test_grcov(&setup).call();

    let result = grcov.compute_coverage(Cancellation::default());

    assert_matches!(result, Err(CoverageError::NoProfrawFiles(details)) =>
    {
        assert_eq!(details.expected_path, setup.coverage_path());
        assert_matches!(details.kind, NoProfrawFilesKind::Io(io_error) =>
        {
           assert_matches!(io_error, std::io::ErrorKind::NotFound);
        });
    });

    Ok(())
}

#[test]
pub fn no_coverage_related_files_are_generated_when_coverage_is_disabled() -> Result<(), IoError>
{
    let setup = TestDataSetup::builder(test_name!(), "simple_project")
        .build()
        .clean_output();

    helpers::test_hyp_run_handler(&setup)
        .coverage_enabled(false)
        .call()
        .handle(HypRunTrigger::DefaultRun, Cancellation::default());

    let profraw_count = get_profraw_count(&setup.coverage_path())?;
    assert_eq!(0, profraw_count);

    let exists = fs::exists(expected_lcov_path(&setup.output_path()))?;
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
