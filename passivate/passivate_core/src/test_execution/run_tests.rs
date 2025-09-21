use std::rc::Rc;

use passivate_delegation::Cancellation;

use super::TestRunError;

#[mockall::automock]
pub trait RunTests
{
    fn run_tests(
        &self,
        instrument_coverage: bool,
        cancellation: Cancellation
    ) -> Result<Box<dyn Iterator<Item = Result<Rc<String>, TestRunError>>>, TestRunError>;

    fn run_test(
        &self,
        test_name: &str,
        update_snapshots: bool,
        cancellation: Cancellation
    ) -> Result<Box<dyn Iterator<Item = Result<Rc<String>, TestRunError>>>, TestRunError>;
}

pub fn mock_run_tests() -> Box<MockRunTests>
{
    Box::new(MockRunTests::new())
}

pub fn stub_run_tests() -> Box<MockRunTests>
{
    use std::iter;

    let mut mock = mock_run_tests();
    mock.expect_run_tests()
        .returning(|_instrument_coverage, _cancellation| Ok(Box::new(iter::empty())));

    mock
}
