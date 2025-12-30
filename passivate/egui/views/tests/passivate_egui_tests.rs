use egui_kittest::Harness;
use passivate_core::passivate_state::PassivateState;
use passivate_delegation::Rx;
use passivate_egui_core::passivate_view_state::PassivateViewState;
use passivate_egui_docking::dock_views::DockViews;
use passivate_egui_views::passivate_views::PassivateViews;
use passivate_egui_views::{passivate_layout, passivate_ui};
use passivate_hyp_names::test_name;
use passivate_model_bridge::hyp_run_bridge::MockHypRunBridge;
use passivate_model_rust::RustBridge;

#[test]
pub fn the_default_layout_looks_like_this()
{
    let view_state = PassivateViewState::<RustBridge>::default();
    let passivate_state = PassivateState::new(Rx::stub());
    let views: PassivateViews<RustBridge, MockHypRunBridge> = PassivateViews::<RustBridge, MockHypRunBridge>::stub();
    let mut layout = passivate_layout::default(&views);
    let mut dock_views = DockViews::new(views.into());

    let mut ui = Harness::new_ui(|ui: &mut egui::Ui| {
        passivate_ui::ui(&view_state, &passivate_state, ui.ctx(), &mut dock_views, &mut layout);
    });

    ui.step();
    ui.snapshot(&test_name!());
}
