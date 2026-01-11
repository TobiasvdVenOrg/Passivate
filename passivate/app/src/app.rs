use eframe::Frame;
use egui::Context;
use passivate_configuration::configuration_source::FileConfigurationSource;
use passivate_delegation::CancellableMessage;
use passivate_egui_docking::docking_layout::DockingLayout;
use passivate_egui_docking::layout_management::LayoutManagement;
use passivate_log::log_message::LogMessage;
use passivate_model_bridge::hyp_run_request::HypRunRequest;
use passivate_model_bridge::hyp_session_event::HypSessionEvent;
use passivate_model_rust::RustBridge;

use crate::app_state::AppState;

pub struct App<'a>
{
    layout: LayoutManagement<FileConfigurationSource<DockingLayout>>,
    state: &'a mut AppState,
    run_hyps_tx: crossbeam_channel::Sender<CancellableMessage<HypRunRequest<RustBridge>>>,
    session_event_rx: crossbeam_channel::Receiver<HypSessionEvent<RustBridge>>,
    log_rx: crossbeam_channel::Receiver<LogMessage>
}

impl<'a> App<'a>
{
    pub fn new(
        layout: LayoutManagement<FileConfigurationSource<DockingLayout>>,
        state: &'a mut AppState,
        run_hyps_tx: crossbeam_channel::Sender<CancellableMessage<HypRunRequest<RustBridge>>>,
        session_event_rx: crossbeam_channel::Receiver<HypSessionEvent<RustBridge>>,
        log_rx: crossbeam_channel::Receiver<LogMessage>
    ) -> Self
    {
        Self {
            layout,
            state,
            run_hyps_tx,
            session_event_rx,
            log_rx
        }
    }

    fn main_update(&mut self, ctx: &Context)
    {
        self.state.update_app(
            ctx,
            self.layout.get_current(),
            &self.run_hyps_tx,
            &self.session_event_rx,
            &self.log_rx
        );
    }
}

impl eframe::App for App<'_>
{
    fn update<'a>(&mut self, ctx: &Context, _frame: &mut Frame)
    {
        self.main_update(ctx);
    }
}
