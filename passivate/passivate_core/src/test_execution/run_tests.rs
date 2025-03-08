use std::sync::mpsc::Sender;
use super::TestRun;
use std::io::Error as IoError;

#[cfg_attr(feature = "mocks", mockall::automock)]
pub trait RunTests {
    fn run_tests(&mut self, sender: &Sender<TestRun>) -> Result<(), IoError>;
}