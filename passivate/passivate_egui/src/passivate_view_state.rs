use crate::snapshots::snapshot_handles::SnapshotHandles;

#[derive(Default)]
pub struct PassivateViewState
{
    snapshot_handles: Option<SnapshotHandles>
}
