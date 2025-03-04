use std::sync::mpsc::Sender;
use super::{RunTestsError, TestsStatus};

#[cfg_attr(feature = "mocks", mockall::automock)]
pub trait RunTests {
    fn run_tests(&self, sender: &Sender<TestsStatus>) -> Result<(), RunTestsError>;
}