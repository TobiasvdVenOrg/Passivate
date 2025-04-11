use std::{iter, rc::Rc};
use crate::{delegation::Cancellation, configuration::TestRunnerImplementation};

use super::TestRunError;

#[mockall::automock]
pub trait RunTests {
    fn run_tests(&self, implementation: TestRunnerImplementation, instrument_coverage: bool, cancellation: Cancellation) 
        -> Result<Box<dyn Iterator<Item = Result<Rc<String>, TestRunError>>>, TestRunError>;

    fn run_test(&self, implementation: TestRunnerImplementation, test_name: &str, update_snapshots: bool, cancellation: Cancellation)
        -> Result<Box<dyn Iterator<Item = Result<Rc<String>, TestRunError>>>, TestRunError>;
}

pub fn mock_run_tests() -> Box<MockRunTests> {
    Box::new(MockRunTests::new())
}

pub fn stub_run_tests() -> Box<MockRunTests> {
    let mut mock = mock_run_tests();
    mock.expect_run_tests().returning(|_implementation, _instrument_coverage, _cancellation| { Ok(Box::new(iter::empty())) });

    mock
}
