use std::path::{Path, PathBuf};
use std::sync::mpsc::Sender;
use crate::change_events::{ChangeEvent, HandleChangeEvent};
use crate::coverage::ComputeCoverage;
use crate::passivate_cargo::*;
use crate::test_execution::TestsStatus;
use crate::passivate_grcov::*;
use std::fs;

pub struct TestRunner {
    path: PathBuf,
    coverage: Box<dyn ComputeCoverage>,
    tests_status_handler: Sender<TestsStatus>
}

impl TestRunner {
    pub fn new(path: &Path, coverage: Box<dyn ComputeCoverage>, tests_status_handler: Sender<TestsStatus>) -> Self {
        TestRunner { path: path.to_path_buf(), coverage, tests_status_handler }
    }
}

impl HandleChangeEvent for TestRunner {
    fn handle_event(&mut self, _event: ChangeEvent) {
        let _ = self.tests_status_handler.send(TestsStatus::running());

        let passivate_path = self.path.join(".passivate");
        let coverage_path = passivate_path.join("coverage");

        remove_profraw_files(&coverage_path).unwrap();
        fs::create_dir_all(&coverage_path).unwrap(); 

        let profraw_path = fs::canonicalize(
            &coverage_path).unwrap().join("coverage-%p-%m.profraw");

        let test_output = cargo_test(&self.path, &profraw_path);

        let binary_path = Path::new("./target/x86_64-pc-windows-msvc/debug/");
        let _lcov_info = grcov(&self.path, &profraw_path, binary_path, &coverage_path);
    
        let status = parse_status(&test_output);
        let _ = self.tests_status_handler.send(status);
    }
}
