use passivate_core::{compose::PassivateCore, startup_errors::StartupError};
use passivate_egui::{configuration_view::ConfigurationView, coverage_view::CoverageView, details_view::DetailsView, docking::dock_views::DockViews, log_view::LogView, passivate_layout, passivate_view_state::PassivateViewState, passivate_views::{PassivateView, PassivateViews}, test_run_view::TestRunView};

use crate::{app::App, app_state::AppState};

pub fn run_app(passivate: PassivateCore) -> Result<(), StartupError>
{
    run_app_and_get_context(passivate, |_| {})
}

pub fn run_app_and_get_context(passivate: PassivateCore, context_accessor: impl FnOnce(egui::Context)) -> Result<(), StartupError>
{
    let PassivateCore { 
        state,
        passivate_path,
        change_event_tx,
        configuration,
        log_rx,
        coverage_rx,
    .. } = passivate;

    // Views
    let tests_view = TestRunView;
    let details_view = DetailsView::new(change_event_tx.clone());
    let coverage_view = CoverageView::new(coverage_rx, configuration.clone());
    let configuration_view = ConfigurationView::new(configuration.clone(), change_event_tx);
    let log_view = LogView::new(log_rx);

    let views = PassivateViews::new(tests_view, details_view, coverage_view, configuration_view, log_view);

    let layout = passivate_layout::load(
        &passivate_path.join("default_layout.toml"),
        &views
    )?;

    let dock_views = DockViews::new(views.into());

    log::info!("Passivate started.");

    // Block until app closes
    let eframe_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_position([1920.0, 0.0])
            .with_inner_size([1024.0, 512.0])
            .with_min_inner_size([300.0, 220.0]),
        ..Default::default()
    };

    let view_state = PassivateViewState::default();
    let mut app_state = AppState::new(state, view_state, dock_views, configuration);

    eframe::run_native(
        "Passivate",
        eframe_options,
        Box::new(|cc| {
            context_accessor(cc.egui_ctx.clone());

            Ok(Box::new(App::new(
                layout,
                &mut app_state
            )))
        })
    )
    .expect("Failed to start Passivate!");

    Ok(())
}
