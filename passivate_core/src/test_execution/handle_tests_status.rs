use crate::test_execution::TestsStatus;

pub trait HandleTestsStatus : Send {
    fn refresh(&mut self, tests_status: TestsStatus);
}