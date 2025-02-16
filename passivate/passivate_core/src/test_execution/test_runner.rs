use std::path::{Path, PathBuf};
use std::sync::mpsc::Sender;
use crate::change_events::{ChangeEvent, HandleChangeEvent};
use crate::coverage::ComputeCoverage;
use crate::passivate_cargo::*;
use crate::test_execution::TestsStatus;
use std::fs;

use super::RunTests;

pub struct TestRunner {
    workspace_path: PathBuf,
    passivate_path: PathBuf,
    coverage_path: PathBuf,
    runner: Box<dyn RunTests>,
    coverage: Box<dyn ComputeCoverage>,
    tests_status_handler: Sender<TestsStatus>
}

impl TestRunner {
    pub fn new(
        workspace_path: &Path, 
        runner: Box<dyn RunTests>,
        coverage: Box<dyn ComputeCoverage>, 
        tests_status_handler: Sender<TestsStatus>) -> Self {
        let passivate_path = workspace_path.join(".passivate");
        let coverage_path = passivate_path.join("coverage");
        TestRunner { 
            workspace_path: workspace_path.to_path_buf(), 
            passivate_path,
            coverage_path,
            runner, 
            coverage, 
            tests_status_handler 
        }
    }
}

impl HandleChangeEvent for TestRunner {
    fn handle_event(&mut self, _event: ChangeEvent) {
        let _ = self.tests_status_handler.send(TestsStatus::running());

        let _ = self.coverage.clean_coverage_output();
        fs::create_dir_all(&self.coverage_path).unwrap(); 

        let test_output = self.runner.run_tests(&self.workspace_path, &self.coverage_path).unwrap();

        
        let _ = self.coverage.compute_coverage();
    
        let status = parse_status(&test_output);
        let _ = self.tests_status_handler.send(status);
    }
}
