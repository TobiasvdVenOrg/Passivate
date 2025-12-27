use std::rc::Weak;

use passivate_configuration::configuration_manager::ConfigurationManager;
use passivate_core::passivate_state_change::PassivateStateChange;
use passivate_egui_hyp_snapshots::Snapshots;
use passivate_egui_hyp_snapshots::snapshot_handles::SnapshotHandles;
use passivate_model_bridge::Bridge;
use passivate_model_core::hyp::Hyp;
use passivate_model_rust::RustBridge;

#[derive(Default)]
pub struct PassivateViewState
{
    pub hyp_details: Option<HypDetails>
}

impl PassivateViewState
{
    pub fn update(
        &mut self,
        change: &PassivateStateChange<RustBridge>,
        configuration: &ConfigurationManager,
        egui_context: &egui::Context
    )
    {
        match change
        {
            PassivateStateChange::HypSelected(hyp) =>
            {
                let hyp = (*hyp).clone();
                let mut hyp_details = HypDetails::new(hyp, None);

                Self::check_for_snapshots(&mut hyp_details, configuration, egui_context);

                self.hyp_details = Some(hyp_details);
            }
            PassivateStateChange::HypDetailsChanged(hyp) =>
            {
                if let Some(details) = &mut self.hyp_details
                    && details.hyp.id == hyp.id
                {
                    details.hyp = (*hyp).clone();

                    Self::check_for_snapshots(details, configuration, egui_context);
                }
            }
        };
    }

    fn check_for_snapshots(details: &mut HypDetails, configuration: &ConfigurationManager, egui_context: &egui::Context)
    {
        let snapshot_directories = configuration.get(|c| c.snapshot_directories.clone());

        if !snapshot_directories.is_empty()
            && let Some(hyp) = details.hyp.upgrade()
        {
            let hyp_id = &hyp.info.id;
            let snapshot = Snapshots::new(snapshot_directories).from_hyp(hyp_id);
            let snapshot_handles = SnapshotHandles::new(hyp_id, snapshot, egui_context);

            details.snapshot_handles = Some(snapshot_handles);
        }
    }
}

pub struct HypDetails
{
    pub hyp: Weak<Hyp<RustBridge>>,
    pub snapshot_handles: Option<SnapshotHandles>
}

impl HypDetails
{
    pub fn new(hyp: Weak<Hyp<RustBridge>>, snapshot_handles: Option<SnapshotHandles>) -> Self
    {
        Self { hyp, snapshot_handles }
    }
}
