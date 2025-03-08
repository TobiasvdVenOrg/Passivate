use std::sync::mpsc::Sender;
use super::{RunTestsError, TestRun};

#[cfg_attr(feature = "mocks", mockall::automock)]
pub trait RunTests {
    fn run_tests(&mut self, sender: &Sender<TestRun>) -> Result<(), RunTestsError>;
}