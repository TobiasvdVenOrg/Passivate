use epaint::TextureHandle;
use passivate_hyp_names::hyp_id::HypId;

use crate::snapshots::SnapshotError;

pub struct SnapshotHandles
{
    pub current: Option<Result<TextureHandle, SnapshotError>>,
    pub new: Option<Result<TextureHandle, SnapshotError>>,
    pub are_identical: bool,
    pub hyp_id: HypId
}
