use passivate_egui::passivate_view_state::PassivateViewState;
use passivate_hyp_model::{single_test::SingleTest, test_run::TestRun};

pub struct PassivateState
{
    pub persisted: PersistedPassivateState,
    pub view: PassivateViewState
}

pub struct PersistedPassivateState
{
    pub hyp_run: TestRun,
    pub selected_hyp: Option<SingleTest>
}
