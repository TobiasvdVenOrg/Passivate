use std::ops::Deref;

use egui::Ui;
use passivate_model_rust::PackageInfo;

use crate::specialize_session_ui::SpecializeSessionUi;

impl SpecializeSessionUi for PackageInfo
{
    fn ui(&self, ui: &mut Ui)
    {
        ui.label(self.package_id.deref());
        ui.label(self.manifest_path.as_str());
    }
}
