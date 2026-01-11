use maybe_owned::MaybeOwned;
use passivate_delegation::{MockRx, Rx, RxError};
use passivate_egui_docking::docking_layout::DockingLayout;
use passivate_log::log_message::LogMessage;
use passivate_model_bridge::hyp_session_event::HypSessionEvent;
use passivate_model_rust::RustBridge;

use crate::app_state::AppState;

pub struct UpdateApp<'a, TRxSession, TRxLog>
{
    test_self: &'a mut AppState,
    egui_context: &'a egui::Context,
    layout: &'a mut DockingLayout,
    session_rx: MaybeOwned<'a, TRxSession>,
    log_rx: MaybeOwned<'a, TRxLog>
}

impl<'a> UpdateApp<'a, MockRx<HypSessionEvent<RustBridge>>, MockRx<LogMessage>>
{
    pub fn new(test_self: &'a mut AppState, egui_context: &'a egui::Context, layout: &'a mut DockingLayout) -> Self
    {
        let mut mock_session_rx = MockRx::new();
        mock_session_rx
            .expect_recv()
            .returning(|| Err(RxError::Recv(crossbeam_channel::RecvError)));
        mock_session_rx
            .expect_try_recv()
            .returning(|| Err(RxError::TryRecv(crossbeam_channel::TryRecvError::Empty)));

        let mut mock_log_rx = MockRx::new();
        mock_log_rx
            .expect_recv()
            .returning(|| Err(RxError::Recv(crossbeam_channel::RecvError)));
        mock_log_rx
            .expect_try_recv()
            .returning(|| Err(RxError::TryRecv(crossbeam_channel::TryRecvError::Empty)));

        Self {
            test_self,
            egui_context,
            layout,
            session_rx: MaybeOwned::Owned(mock_session_rx),
            log_rx: MaybeOwned::Owned(mock_log_rx)
        }
    }
}

impl<'a, TRxSession, TRxLog> UpdateApp<'a, TRxSession, TRxLog>
where
    TRxSession: Rx<HypSessionEvent<RustBridge>>,
    TRxLog: Rx<LogMessage>
{
    pub fn call(&mut self)
    {
        self.test_self
            .update_app(self.egui_context, self.layout, self.session_rx.as_ref(), self.log_rx.as_ref());
    }
}
