use passivate_hyp_model::test_run::TestRun;
use passivate_hyp_names::hyp_id::HypId;

pub struct PassivateState
{
    pub persisted: PersistedPassivateState
}

pub struct PersistedPassivateState
{
    pub hyp_run: TestRun,
    pub selected_hyp: Option<HypId>
}
