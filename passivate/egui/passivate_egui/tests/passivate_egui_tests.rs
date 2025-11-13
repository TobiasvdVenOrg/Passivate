use egui_kittest::Harness;
use passivate_core::passivate_state::PassivateState;
use passivate_delegation::Rx;
use passivate_egui::{docking::dock_views::DockViews, passivate_layout, passivate_view_state::PassivateViewState, passivate_views::PassivateViews};
use passivate_hyp_names::test_name;


#[test]
pub fn the_default_layout_looks_like_this()
{
    let view_state = PassivateViewState::default();
    let passivate_state = PassivateState::new(Rx::stub());
    let views = PassivateViews::stub();
    let mut layout = passivate_layout::default(&views);
    let mut dock_views = DockViews::new(views.into());
    
    let mut ui = Harness::new_ui(|ui: &mut egui::Ui| {
        view_state.ui(&passivate_state, ui.ctx(), &mut dock_views, &mut layout);
    });

    ui.step();
    ui.snapshot(&test_name!());
}
