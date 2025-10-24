use crate::{single_test::SelectedHyp, test_run::TestRun};

pub struct PassivateState
{
    pub hyp_run: TestRun,
    pub selected_hyp: Option<SelectedHyp>
}
