use crate::single_test::SingleTest;
use crate::test_run::TestRun;

pub struct PassivateState
{
    pub hyp_run: TestRun,
    pub selected_hyp: Option<SingleTest>
}
