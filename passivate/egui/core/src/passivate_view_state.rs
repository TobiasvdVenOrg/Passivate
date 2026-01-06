use passivate_configuration::configuration::PassivateConfiguration;
use passivate_configuration::configuration_manager::ConfigurationManager;
use passivate_core::passivate_state_change::PassivateStateChange;
use passivate_delegation::Rx;
use passivate_egui_hyp_snapshots::Snapshots;
use passivate_egui_hyp_snapshots::snapshot_handles::SnapshotHandles;
use passivate_log::log_message::LogMessage;
use passivate_model_bridge::bridge::Bridge;
use passivate_model_core::hyp::Hyp;

use crate::log_entry::LogEntry;

pub struct PassivateViewState<TBridge: Bridge>
{
    snapshot_handles: Option<SnapshotHandles<TBridge::Id>>,
    logs: Vec<LogEntry>
}

impl<TBridge: Bridge> Default for PassivateViewState<TBridge>
{
    fn default() -> Self
    {
        Self {
            snapshot_handles: None,
            logs: Vec::new()
        }
    }
}

impl<TBridge: Bridge> PassivateViewState<TBridge>
{
    pub fn update_view_state(
        &mut self,
        change: &PassivateStateChange<TBridge>,
        configuration: &PassivateConfiguration,
        egui_context: &egui::Context,
        logs_rx: &impl Rx<LogMessage>
    )
    {
        if let Ok(log) = logs_rx.try_recv()
        {
            self.logs.push(LogEntry::from(log));
        }

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
            PassivateStateChange::ConfigurationChanged(_) =>
            {}
        };
    }

    pub fn snapshot_handles(&self) -> Option<&SnapshotHandles<TBridge::Id>>
    {
        self.snapshot_handles.as_ref()
    }

    pub fn logs(&self) -> &Vec<LogEntry>
    {
        &self.logs
    }

    fn check_for_snapshots(
        hyp: &Hyp<TBridge>,
        configuration: &PassivateConfiguration,
        egui_context: &egui::Context
    ) -> Option<SnapshotHandles<TBridge::Id>>
    {
        let snapshot_directories = &configuration.snapshot_directories;

        if !snapshot_directories.is_empty()
        {
            let snapshot = Snapshots::new(snapshot_directories.clone()).from_hyp(hyp);
            let snapshot_handles = SnapshotHandles::new(hyp.id().clone(), snapshot, egui_context);

            return Some(snapshot_handles);
        }

        None
    }
}
