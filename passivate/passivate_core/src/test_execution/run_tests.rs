use std::path::Path;
use super::RunTestsError;

pub trait RunTests {
    fn run_tests(&self, path: &Path, profraw_output_dir: &Path) -> Result<String, RunTestsError>;
}