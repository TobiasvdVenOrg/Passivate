#[cfg(test)]
use mockall::*;

use super::RunTestsError;

#[cfg_attr(test, automock)]
pub trait RunTests {
    fn run_tests(&self) -> Result<String, RunTestsError>;
}