use std::io::Error as IoError;
use crate::configuration::TestRunnerImplementation;

#[mockall::automock]
pub trait RunTests {
    fn run_tests(&self, implementation: TestRunnerImplementation) -> Result<Box<dyn Iterator<Item = Result<String, IoError>>>, IoError>;
}