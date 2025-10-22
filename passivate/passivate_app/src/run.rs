use passivate_core::{passivate_args::PassivateArgs, run::{run, PassivateCore}, startup_errors::StartupError};
use passivate_delegation::Tx;
use passivate_views::{configuration_view::ConfigurationView, coverage_view::CoverageView, details_view::DetailsView, docking::{tab_viewer::TabViewer, view::View}, log_view::LogView, passivate_layout, test_run_view::TestRunView};

use crate::app::App;

// Called by main
pub fn run_app(passivate: PassivateCore) -> Result<(), StartupError>
{
    run_app_and_get_context(passivate, |_| {})
}

// Called by passivate_tests
pub fn run_with_args(args: PassivateArgs, context_accessor: impl FnOnce(egui::Context)) -> Result<(), StartupError>
{
    run(args, |passivate| {
        run_app_and_get_context(passivate, context_accessor)
    })
}

pub fn run_app_and_get_context(passivate: PassivateCore, context_accessor: impl FnOnce(egui::Context)) -> Result<(), StartupError>
{
    let PassivateCore { 
        passivate_path,
        change_event_tx,
        configuration,
        log_rx,
        hyp_run_rx,
        coverage_rx,
        test_run } = passivate;

    let (details_tx, details_rx) = Tx::new();

    // Views
    let tests_view = TestRunView::new(test_run, hyp_run_rx, details_tx);
    let details_view = DetailsView::new(details_rx, change_event_tx.clone(), configuration.clone());
    let coverage_view = CoverageView::new(coverage_rx, configuration.clone());
    let configuration_view = ConfigurationView::new(configuration, change_event_tx);
    let log_view = LogView::new(log_rx);

    let layout = passivate_layout::load(
        &passivate_path.join("default_layout.toml"),
        &tests_view,
        &details_view,
        &coverage_view,
        &configuration_view,
        &log_view
    )?;

    let views: Vec<Box<dyn View>> = vec![
        Box::new(tests_view),
        Box::new(details_view),
        Box::new(coverage_view),
        Box::new(configuration_view),
        Box::new(log_view),
    ];

    let tab_viewer = TabViewer::new(views.into_iter());

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
                tab_viewer
            )))
        })
    )
    .expect("Failed to start Passivate!");

    Ok(())
}
