use std::io::Error as IoError;
use std::path::{Path, PathBuf};
use std::fs;
mod helpers;
use helpers::*;
use rstest::*;

#[cfg(target_os = "windows")]
#[rstest]
#[case::cargo(cargo_builder())]
#[case::nextest(nextest_builder())]
pub fn test_run_outputs_coverage_file_for_project(#[case] mut builder: TestRunnerBuilder) -> Result<(), IoError> {
    let mut runner = builder
        .with_workspace("simple_project")
        .with_output("test_run_outputs_coverage_file_for_project")
        .build()?;

    test_run(&mut runner)?;

    let file_data = expected_lcov_metadata(&builder.get_output_path());
    assert!(file_data.is_ok(), "Expected coverage output file did not exist: {:?}", file_data);

    Ok(())
}

#[cfg(target_os = "windows")]
#[rstest]
#[case::cargo(cargo_builder())]
#[case::nextest(nextest_builder())]
pub fn test_run_outputs_coverage_file_for_workspace(#[case] mut builder: TestRunnerBuilder) -> Result<(), IoError> {
    let mut runner = builder
        .with_workspace("simple_workspace")
        .with_output("test_run_outputs_coverage_file_for_workspace")
        .build()?;

    test_run(&mut runner)?;

    let file_data = expected_lcov_metadata(&builder.get_output_path());
    assert!(file_data.is_ok(), "Expected coverage output file did not exist: {:?}", file_data);

    Ok(())
}

#[cfg(target_os = "windows")]
#[rstest]
#[case::cargo(cargo_builder())]
#[case::nextest(nextest_builder())]
pub fn repeat_test_runs_do_not_accumulate_profraw_files(#[case] mut builder: TestRunnerBuilder) -> Result<(), IoError> {
    let mut runner = builder
        .with_workspace("simple_workspace")
        .with_output("repeat_test_runs_do_not_accumulate_profraw_files")
        .build()?;

    test_run(&mut runner)?;

    let first_run = get_profraw_count(&builder.get_output_path())?;

    test_run(&mut runner)?;

    let second_run = get_profraw_count(&builder.get_output_path())?; 

    assert_ne!(0, second_run);
    assert_eq!(first_run, second_run);
    Ok(())
}

#[cfg(target_os = "windows")]
#[rstest]
#[case::cargo(cargo_builder())]
#[case::nextest(nextest_builder())]
// Temporary deletion of the lcov.info file before re-creation can cause coverage systems relying on it (like Coverage Gutters in VSCode)
// to briefly error due to "not finding the file" until a new one is created
pub fn repeat_test_runs_do_not_delete_lcov_file(#[case] mut builder: TestRunnerBuilder) -> Result<(), IoError> {
    let mut runner = builder
        .with_workspace("simple_workspace")
        .with_output("repeat_test_runs_do_not_accumulate_profraw_files")
        .build()?;

    test_run(&mut runner)?;

    let first_run_metadata = expected_lcov_metadata(&builder.get_output_path())?;

    test_run(&mut runner)?;

    let second_run_metadata = expected_lcov_metadata(&builder.get_output_path())?;
    
    assert_eq!(first_run_metadata.created()?, second_run_metadata.created()?);
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

fn expected_lcov_path(test_name: &Path) -> PathBuf {
    test_output_path().join(test_name).join(".passivate/coverage/lcov.info")
}

fn expected_lcov_metadata(test_name: &Path) -> Result<fs::Metadata, IoError> {
    let path = expected_lcov_path(test_name); 
    fs::metadata(path)
}
