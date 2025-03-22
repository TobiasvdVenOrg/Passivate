use std::{io::Error as IoError, iter};
use crate::configuration::TestRunnerImplementation;

use super::TestRunError;

#[mockall::automock]
pub trait RunTests {
    fn run_tests(&self, implementation: TestRunnerImplementation, instrument_coverage: bool) -> Result<Box<dyn Iterator<Item = Result<String, IoError>>>, TestRunError>;
}

pub fn mock_run_tests() -> Box<MockRunTests> {
    Box::new(MockRunTests::new())
}

pub fn stub_run_tests() -> Box<MockRunTests> {
    let mut mock = mock_run_tests();
    mock.expect_run_tests().returning(|_implementation, _instrument_coverage| { Ok(Box::new(iter::empty())) });

    mock
}
