use egui::Ui;
use passivate_model_rust::HypPackage;

use crate::specialize_session_ui::SpecializeSessionUi;

impl SpecializeSessionUi for HypPackage
{
    fn ui(&self, ui: &mut Ui)
    {
        ui.label(&self.package_name);
        ui.label(self.manifest_path.as_str());
    }
}
