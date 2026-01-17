use std::fmt::Debug;

use epaint::TextureHandle;
use epaint::textures::TextureOptions;

use crate::{Snapshot, SnapshotError};

pub struct SnapshotHandles<THypId>
{
    pub current: Option<Result<TextureHandle, SnapshotError>>,
    pub new: Option<Result<TextureHandle, SnapshotError>>,
    pub are_identical: bool,
    pub hyp_id: THypId
}

impl<THypId> SnapshotHandles<THypId>
{
    pub fn new(hyp_id: THypId, snapshot: Snapshot, egui_context: &egui::Context) -> Self
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

    pub fn empty(hyp_id: THypId) -> Self
    {
        SnapshotHandles {
            current: None,
            new: None,
            are_identical: true,
            hyp_id
        }
    }
}

impl<THypId: Debug> Debug for SnapshotHandles<THypId>
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
    {
        f.debug_struct("SnapshotHandles")
            .field("current", &self.current.is_some())
            .field("new", &self.new.is_some())
            .field("are_identical", &self.are_identical)
            .field("hyp_id", &self.hyp_id)
            .finish()
    }
}
