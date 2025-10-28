use passivate_core::passivate_state::PassivateState;
use passivate_hyp_model::{hyp_run_events::HypRunChange, single_hyp::SingleHyp};

use crate::snapshots::snapshot_handles::SnapshotHandles;

#[derive(Default)]
pub struct PassivateViewState
{
    pub hyp_details: Option<HypDetails>
}

impl PassivateViewState
{
    // pub fn update(&mut self, state: &PassivateState, change: Option<&HypRunChange<'_>>)
    // {
    //     if let Some(details) = &mut self.hyp_details
    //         && let Some(hyp) = state.hyp_run.tests.find(&details.hyp.id)
    //         {
    //             details.hyp = hyp.clone();
    //         }
    // }
}

pub struct HypDetails
{
    pub hyp: SingleHyp,
    pub snapshot_handles: Option<SnapshotHandles>
}

impl HypDetails
{
    pub fn new(hyp: SingleHyp, snapshot_handles: Option<SnapshotHandles>) -> Self
    {
        Self { hyp, snapshot_handles }
    }
}
