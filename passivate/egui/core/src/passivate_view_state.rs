use passivate_configuration::configuration_manager::ConfigurationManager;
use passivate_core::passivate_state_change::PassivateStateChange;
use passivate_egui_hyp_snapshots::Snapshots;
use passivate_egui_hyp_snapshots::snapshot_handles::SnapshotHandles;
use passivate_model_bridge::bridge::Bridge;
use passivate_model_core::hyp::Hyp;

pub struct PassivateViewState<TBridge: Bridge>
{
    snapshot_handles: Option<SnapshotHandles<TBridge::Id>>
}

impl<TBridge: Bridge> Default for PassivateViewState<TBridge>
{
    fn default() -> Self
    {
        Self { snapshot_handles: None }
    }
}

impl<TBridge: Bridge> PassivateViewState<TBridge>
{
    pub fn update(
        &mut self,
        change: &PassivateStateChange<TBridge>,
        configuration: &ConfigurationManager,
        egui_context: &egui::Context
    )
    {
        match change
        {
            PassivateStateChange::HypSelected(hyp) =>
            {
                self.snapshot_handles = Self::check_for_snapshots(hyp, configuration, egui_context);
            }
            PassivateStateChange::HypDetailsChanged(hyp) =>
            {
                if let Some(snapshot_handles) = &mut self.snapshot_handles
                    && snapshot_handles.hyp_id == *hyp.id()
                {
                    self.snapshot_handles = Self::check_for_snapshots(hyp, configuration, egui_context);
                }
            }
        };
    }

    pub fn snapshot_handles(&self) -> Option<&SnapshotHandles<TBridge::Id>>
    {
        self.snapshot_handles.as_ref()
    }

    fn check_for_snapshots(
        hyp: &Hyp<TBridge>,
        configuration: &ConfigurationManager,
        egui_context: &egui::Context
    ) -> Option<SnapshotHandles<TBridge::Id>>
    {
        let snapshot_directories = configuration.get(|c| c.snapshot_directories.clone());

        if !snapshot_directories.is_empty()
        {
            let snapshot = Snapshots::new(snapshot_directories).from_hyp(hyp);
            let snapshot_handles = SnapshotHandles::new(hyp.id().clone(), snapshot, egui_context);

            return Some(snapshot_handles);
        }

        None
    }
}
