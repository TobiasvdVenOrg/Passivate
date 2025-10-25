use epaint::{textures::TextureOptions, TextureHandle};
use passivate_hyp_names::hyp_id::HypId;

use crate::snapshots::{Snapshot, SnapshotError};

pub struct SnapshotHandles
{
    pub current: Option<Result<TextureHandle, SnapshotError>>,
    pub new: Option<Result<TextureHandle, SnapshotError>>,
    pub are_identical: bool,
    pub hyp_id: HypId
}

impl SnapshotHandles
{
    pub fn new(hyp_id: HypId, snapshot: Snapshot, egui_context: egui::Context) -> Self
    {
        let mut are_identical = false;

        if let (Some(Ok(current)), Some(Ok(new))) = (&snapshot.current, &snapshot.new)
        {
            are_identical = current == new;
        }

        let current = snapshot
            .current
            .map(|current| current.map(|s| egui_context.load_texture("current_snapshot", s, TextureOptions::LINEAR)));
        
        let new = snapshot
            .new
            .map(|new| new.map(|s| egui_context.load_texture("new_snapshot", s, TextureOptions::LINEAR)));

        SnapshotHandles {
            current,
            new,
            are_identical,
            hyp_id
        }
    }

    pub fn empty(hyp_id: HypId) -> Self
    {
        SnapshotHandles { 
            current: None, 
            new: None, 
            are_identical: true, 
            hyp_id 
        }
    }
}
