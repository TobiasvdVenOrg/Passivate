use passivate_core::compose::PassivateCore;
use passivate_core::startup_errors::StartupError;
use passivate_egui_core::passivate_view_state::PassivateViewState;
use passivate_egui_docking::dock_views::DockViews;
use passivate_egui_view_configuration::ConfigurationView;
use passivate_egui_view_coverage::CoverageView;
use passivate_egui_view_details::details_view::DetailsView;
use passivate_egui_view_log::LogView;
use passivate_egui_view_session::session_view::SessionView;
use passivate_egui_views::passivate_layout;
use passivate_egui_views::passivate_views::PassivateViews;

use crate::app::App;
use crate::app_state::AppState;

pub fn run_app(passivate: PassivateCore) -> Result<(), StartupError>
{
    run_app_and_get_context(passivate, |_| {})
}

pub fn run_app_and_get_context(
    passivate: PassivateCore,
    context_accessor: impl FnOnce(egui::Context)
) -> Result<(), StartupError>
{
    let PassivateCore {
        session,
        state,
        passivate_path,
        source_change_rx,
        hyp_run_tx,
        session_event_rx,
        configuration,
        log_rx,
        ..
    } = passivate;

    // Views
    let tests_view = SessionView;
    let details_view = DetailsView;
    let coverage_view = CoverageView;
    let configuration_view = ConfigurationView::new();
    let log_view = LogView;

    let views = PassivateViews::new(tests_view, details_view, coverage_view, configuration_view, log_view);

    let layout = passivate_layout::load(&passivate_path.join("default_docking_layout.toml"), &views)?;
    let dock_views = DockViews::new(views.into());

    log::info!("Passivate started.");

    // Block until app closes
    let eframe_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_position([1920.0, 0.0])
            .with_inner_size([1024.0, 512.0])
            .with_min_inner_size([300.0, 220.0]),
        persist_window: true,
        persistence_path: Some(passivate_path.join("default_window_state.json").into_std_path_buf()),
        ..Default::default()
    };

    let view_state = PassivateViewState::default();
    let mut app_state = AppState::new(session, state, view_state, dock_views, configuration);

    eframe::run_native(
        "Passivate",
        eframe_options,
        Box::new(|cc| {
            context_accessor(cc.egui_ctx.clone());

            Ok(Box::new(App::new(
                layout,
                &mut app_state,
                hyp_run_tx,
                source_change_rx,
                session_event_rx,
                log_rx
            )))
        })
    )
    .expect("Failed to start Passivate!");

    Ok(())
}
