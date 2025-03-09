use std::sync::mpsc::Sender;
use std::io::Error as IoError;
use crate::test_run_model::TestRun;

#[cfg_attr(feature = "mocks", mockall::automock)]
pub trait RunTests {
    fn run_tests(&mut self, sender: &Sender<TestRun>) -> Result<(), IoError>;
}