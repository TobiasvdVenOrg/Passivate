use std::io::Error as IoError;
use crate::configuration::TestRunnerImplementation;

use super::TestRunIterator;

#[mockall::automock]
pub trait RunTests {
    fn run_tests(&self, implementation: TestRunnerImplementation) -> Result<TestRunIterator, IoError>;
}