use passivate_core::passivate_state::PassivateState;
use passivate_hyp_model::single_test::SingleTest;

use crate::snapshots::snapshot_handles::SnapshotHandles;

#[derive(Default)]
pub struct PassivateViewState
{
    pub hyp_details: Option<HypDetails>
}

impl PassivateViewState
{
    pub fn update(&mut self, state: &PassivateState)
    {
        if let Some(details) = &mut self.hyp_details
            && let Some(hyp) = state.hyp_run.tests.find(&details.hyp.id)
            {
                details.hyp = hyp.clone();
            }
    }
}

pub struct HypDetails
{
    pub hyp: SingleTest,
    pub snapshot_handles: Option<SnapshotHandles>
}

impl HypDetails
{
    pub fn new(hyp: SingleTest, snapshot_handles: Option<SnapshotHandles>) -> Self
    {
        Self { hyp, snapshot_handles }
    }
}
