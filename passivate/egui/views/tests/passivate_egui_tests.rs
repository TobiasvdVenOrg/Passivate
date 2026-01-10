use egui_kittest::Harness;
use passivate_configuration::configuration::PassivateConfiguration;
use passivate_core::passivate_state::PassivateState;
use passivate_egui_core::passivate_view_state::PassivateViewState;
use passivate_egui_docking::dock_views::DockViews;
use passivate_egui_views::passivate_views::PassivateViews;
use passivate_egui_views::{passivate_layout, passivate_ui};
use passivate_hyp_names::test_name;
use passivate_testing::model::TestSession;

#[test]
pub fn the_default_layout_looks_like_this()
{
    let session = TestSession::new();
    let view_state = PassivateViewState::<TestSession>::default();
    let passivate_state = PassivateState::new();
    let configuration = PassivateConfiguration::default();
    let views = PassivateViews::stub();
    let mut layout = passivate_layout::default(&views);
    let mut dock_views = DockViews::new(views.into());

    let mut ui = Harness::new_ui(|ui: &mut egui::Ui| {
        passivate_ui::ui(
            &session,
            &view_state,
            &passivate_state,
            &configuration,
            ui.ctx(),
            &mut dock_views,
            &mut layout
        );
    });

    ui.step();
    ui.snapshot(&test_name!());
}
