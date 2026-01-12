use egui::Ui;
use passivate_model_core::hyp::Hyp;
use passivate_run_rust::model::{RustBridge, RustHypKind};

use crate::session_view::{self, HypUiAction};
use crate::specialize_session_ui::SpecializeSessionUi;

impl SpecializeSessionUi for Hyp<RustBridge>
{
    fn ui(&self, ui: &mut Ui) -> Option<HypUiAction>
    {
        let info = self.info();
        let name = info.name();

        match &info.kind
        {
            RustHypKind::Single(_) => session_view::show_hyp(ui, self, name),
            RustHypKind::Package(package_info) =>
            {
                let package = format!("{} ({})", name, package_info.manifest_path);
                session_view::show_hyp(ui, self, package)
            }
        }
    }
}
