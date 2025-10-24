use passivate_core::{passivate_args::PassivateArgs, compose::{compose, PassivateCore}, startup_errors::StartupError};
use passivate_views::{configuration_view::ConfigurationView, coverage_view::CoverageView, details_view::DetailsView, docking::dock_views::DockViews, log_view::LogView, passivate_layout, passivate_view::PassivateView, test_run_view::TestRunView};

use crate::app::App;

// Called by main
pub fn run_app(passivate: PassivateCore) -> Result<(), StartupError>
{
    run_app_and_get_context(passivate, |_| {})
}

// Called by passivate_tests
pub fn run_with_args(args: PassivateArgs, context_accessor: impl FnOnce(egui::Context)) -> Result<(), StartupError>
{
    compose(args, |passivate| {
        run_app_and_get_context(passivate, context_accessor)
    })
}

pub fn run_app_and_get_context(passivate: PassivateCore, context_accessor: impl FnOnce(egui::Context)) -> Result<(), StartupError>
{
    let PassivateCore { 
        mut state,
        passivate_path,
        change_event_tx,
        configuration,
        log_rx,
        hyp_run_rx,
        coverage_rx } = passivate;

    // Views
    let tests_view = PassivateView::TestRun(TestRunView);
    let details_view = PassivateView::Details(DetailsView::new(change_event_tx.clone(), configuration.clone()));
    let coverage_view = PassivateView::Coverage(CoverageView::new(coverage_rx, configuration.clone()));
    let configuration_view = PassivateView::Configuration(ConfigurationView::new(configuration, change_event_tx));
    let log_view = PassivateView::Log(LogView::new(log_rx));

    let views = [tests_view, details_view, coverage_view, configuration_view, log_view];

    let layout = passivate_layout::load(
        &passivate_path.join("default_layout.toml"),
        &views
    )?;

    let dock_views = DockViews::new(views);

    log::info!("Passivate started.");

    // Block until app closes
    let eframe_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_position([1920.0, 0.0])
            .with_inner_size([1024.0, 512.0])
            .with_min_inner_size([300.0, 220.0]),
        ..Default::default()
    };

    eframe::run_native(
        "Passivate",
        eframe_options,
        Box::new(|cc| {
            context_accessor(cc.egui_ctx.clone());

            Ok(Box::new(App::new(
                layout,
                dock_views,
                &mut state,
                hyp_run_rx
            )))
        })
    )
    .expect("Failed to start Passivate!");

    Ok(())
}
