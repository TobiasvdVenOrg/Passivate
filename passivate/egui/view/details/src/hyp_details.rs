use passivate_egui_hyp_snapshots::snapshot_handles::SnapshotHandles;
use passivate_model_bridge::bridge::Bridge;
use passivate_model_core::hyp::Hyp;

pub struct HypDetails<'a, TBridge: Bridge>
{
    pub hyp: &'a Hyp<TBridge>,
    pub snapshot_handles: Option<&'a SnapshotHandles<TBridge::Id>>
}
