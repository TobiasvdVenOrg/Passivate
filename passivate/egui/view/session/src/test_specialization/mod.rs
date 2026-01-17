use egui::Ui;
use passivate_model_core::hyp::Hyp;
use passivate_testing::model::TestSession;

use crate::session_view::{self, HypUiAction};
use crate::specialize_session_ui::SpecializeSessionUi;

impl SpecializeSessionUi for Hyp<TestSession>
{
    fn ui(&self, ui: &mut Ui) -> Option<HypUiAction>
    {
        let info = self.info();
        let name = info.to_string();

        session_view::show_hyp(ui, self, name)
    }
}
