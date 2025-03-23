#![cfg(target_os = "windows")]

use std::fs;
use std::io::Error as IoError;
use std::path::{Path, PathBuf};
mod helpers;
use helpers::*;
use passivate_core::actors::Cancellation;
use passivate_core::coverage::{ComputeCoverage, CoverageError, NoProfrawFilesKind};
use passivate_core::passivate_grcov::get_profraw_count;
use rstest::*;
use stdext::function_name;
use std::sync::mpsc::channel;
use passivate_core::coverage::CoverageStatus;
use pretty_assertions::assert_eq;

#[rstest]
#[case::cargo(cargo_builder())]
#[case::nextest(nextest_builder())]
pub fn test_run_sends_coverage_result(#[case] mut builder: ChangeEventHandlerBuilder) -> Result<(), IoError> {
    let (coverage_sender, coverage_receiver) = channel();
    let mut runner = builder
        .with_workspace("simple_project")
        .with_output(function_name!())
        .coverage_enabled(true)
        .receive_coverage_status(coverage_sender)
        .clean_output()
        .build();

    test_run(&mut runner)?;

    let result = coverage_receiver.try_iter().last().unwrap();

    match result {
        CoverageStatus::Disabled => panic!(),
        CoverageStatus::Preparing => panic!(),
        CoverageStatus::Running => panic!(),
        CoverageStatus::Done(covdir_json) => {
            assert_eq!(100.0, covdir_json.coverage_percent);
        },
        CoverageStatus::Error(_) => panic!(),
    };

    Ok(())
}

#[rstest]
#[case::cargo(cargo_builder())]
#[case::nextest(nextest_builder())]
pub fn test_run_outputs_coverage_file_for_project(#[case] mut builder: ChangeEventHandlerBuilder) -> Result<(), IoError> {
    let mut runner = builder
        .with_workspace("simple_project")
        .with_output(function_name!())
        .coverage_enabled(true)
        .clean_output()
        .build();

    test_run(&mut runner)?;

    let file_data = expected_lcov_metadata(&builder.get_output_path());
    assert!(file_data.is_ok(), "Expected coverage output file did not exist: {:?}", file_data);

    Ok(())
}

#[rstest]
#[case::cargo(cargo_builder())]
#[case::nextest(nextest_builder())]
pub fn test_run_outputs_coverage_file_for_workspace(#[case] mut builder: ChangeEventHandlerBuilder) -> Result<(), IoError> {
    let mut runner = builder
        .with_workspace("simple_workspace")
        .with_output(function_name!())
        .coverage_enabled(true)
        .clean_output()
        .build();

    test_run(&mut runner)?;

    let file_data = expected_lcov_metadata(&builder.get_output_path());
    assert!(file_data.is_ok(), "Expected coverage output file did not exist: {:?}", file_data);

    Ok(())
}

#[rstest]
#[case::cargo(cargo_builder())]
#[case::nextest(nextest_builder())]
pub fn repeat_test_runs_do_not_accumulate_profraw_files(#[case] mut builder: ChangeEventHandlerBuilder) -> Result<(), IoError> {
    let mut runner = builder
        .with_workspace("simple_project")
        .with_output(function_name!())
        .coverage_enabled(true)
        .clean_output()
        .build();

    test_run(&mut runner)?;

    let first_run = get_profraw_count(&builder.get_coverage_path())?;

    test_run(&mut runner)?;

    let second_run = get_profraw_count(&builder.get_coverage_path())?;

    assert_ne!(0, second_run);
    assert_eq!(first_run, second_run);
    Ok(())
}

#[rstest]
#[case::cargo(cargo_builder())]
#[case::nextest(nextest_builder())]
// Temporary deletion of the lcov file before re-creation can cause coverage systems relying on it (like Coverage Gutters in VSCode)
// to briefly error due to "not finding the file" until a new one is created
pub fn repeat_test_runs_do_not_delete_lcov_file(#[case] mut builder: ChangeEventHandlerBuilder) -> Result<(), IoError> {
    let mut runner = builder
        .with_workspace("simple_project")
        .with_output(function_name!())
        .coverage_enabled(true)
        .clean_output()
        .build();

    test_run(&mut runner)?;

    let first_run_metadata = expected_lcov_metadata(&builder.get_output_path())?;

    test_run(&mut runner)?;

    let second_run_metadata = expected_lcov_metadata(&builder.get_output_path())?;
    
    assert_eq!(first_run_metadata.created()?, second_run_metadata.created()?);
    Ok(())
}

#[rstest]
pub fn error_when_coverage_is_computed_with_no_profraw_files_present() -> Result<(), IoError> {
    let mut builder = cargo_builder();
    let builder = builder
        .with_workspace("simple_project")
        .with_output(function_name!())
        .coverage_enabled(true)
        .clean_output();

    fs::create_dir_all(builder.get_coverage_path())?;
    let grcov = builder.build_grcov();

    let result = grcov.compute_coverage(Cancellation::default());
    
    assert!(result.is_err_and(|e| {
        match e {
            CoverageError::NoProfrawFiles(details) => {
                assert_eq!(builder.get_coverage_path(), details.expected_path);
                assert_eq!(NoProfrawFilesKind::NoProfrawFilesExist, details.kind);

                true
            }
            _ => false,
        }
    }));

    Ok(())
}

#[rstest]
pub fn error_when_coverage_is_computed_and_profraw_output_directory_does_not_exist() -> Result<(), IoError> {
    let mut builder = cargo_builder();
    let builder = builder
        .with_workspace("simple_project")
        .with_output(function_name!())
        .coverage_enabled(true)
        .clean_output();

    let grcov = builder.build_grcov();

    let result = grcov.compute_coverage(Cancellation::default());
    
    assert!(result.is_err_and(|e| {
        match e {
            CoverageError::NoProfrawFiles(details) => {
                assert_eq!(builder.get_coverage_path(), details.expected_path);

                match details.kind {
                    NoProfrawFilesKind::Io(error_kind) => assert_eq!(std::io::ErrorKind::NotFound, error_kind),
                    _ => panic!(),
                };

                true
            }
            _ => false,
        }
    }));

    Ok(())
}

#[rstest]
#[case::cargo(cargo_builder())]
#[case::nextest(nextest_builder())]
pub fn no_coverage_related_files_are_generated_when_coverage_is_disabled(#[case] mut builder: ChangeEventHandlerBuilder) -> Result<(), IoError> {
    let mut runner = builder
        .with_workspace("simple_project")
        .with_output(function_name!())
        .coverage_enabled(false)
        .clean_output()
        .build();

    test_run(&mut runner)?;

    let profraw_count = get_profraw_count(&builder.get_coverage_path())?;
    assert_eq!(0, profraw_count);
    
    let exists = fs::exists(expected_lcov_path(&builder.get_output_path()))?;
    assert!(!exists, "lcov file existed unexpectedly");

    Ok(())
}

fn expected_lcov_path(test_name: &Path) -> PathBuf {
    test_output_path().join(test_name).join(".passivate/coverage/lcov")
}

fn expected_lcov_metadata(test_name: &Path) -> Result<fs::Metadata, IoError> {
    let path = expected_lcov_path(test_name); 
    fs::metadata(path)
}
