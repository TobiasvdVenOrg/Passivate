use std::sync::mpsc::Sender;
use super::{RunTestsError, TestsStatus};

#[cfg(test)]
use mockall::*;

#[cfg_attr(test, automock)]
pub trait RunTests {
    fn run_tests(&self, sender: &Sender<TestsStatus>) -> Result<TestsStatus, RunTestsError>;
}