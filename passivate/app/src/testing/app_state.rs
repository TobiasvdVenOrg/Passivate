use maybe_owned::MaybeOwned;
use passivate_delegation::{MockRx, Rx, RxError};
use passivate_egui_docking::docking_layout::DockingLayout;
use passivate_log::log_message::LogMessage;
use passivate_model_bridge::bridge::Bridge;
use passivate_model_bridge::hyp_run_bridge::{MockRunHypsBridge, RunHypsBridge};
use passivate_model_bridge::hyp_session_event::HypSessionEvent;
use passivate_model_bridge::source_change_event::SourceChangeEvent;

use crate::app_state::AppState;

pub struct UpdateApp<'a, TBridge: Bridge, TRunHyps, TRxSourceChange, TRxSession, TRxLog>
{
    test_self: &'a mut AppState<TBridge>,
    egui_context: &'a egui::Context,
    layout: &'a mut DockingLayout,
    run_hyps: MaybeOwned<'a, TRunHyps>,
    source_change_rx: MaybeOwned<'a, TRxSourceChange>,
    session_rx: MaybeOwned<'a, TRxSession>,
    log_rx: MaybeOwned<'a, TRxLog>
}

impl<'a, TBridge: Bridge>
    UpdateApp<
        'a,
        TBridge,
        MockRunHypsBridge<TBridge>,
        MockRx<SourceChangeEvent>,
        MockRx<HypSessionEvent<TBridge>>,
        MockRx<LogMessage>
    >
{
    pub fn with(test_self: &'a mut AppState<TBridge>, egui_context: &'a egui::Context, layout: &'a mut DockingLayout) -> Self
    {
        let mut mock_run_hyps = MockRunHypsBridge::new();
        mock_run_hyps.expect_run_all().returning(|_| ());
        mock_run_hyps.expect_run_single().returning(|_, _| ());

        let mut mock_source_change_rx = MockRx::new();
        mock_source_change_rx
            .expect_recv()
            .returning(|| Err(RxError::Recv(crossbeam_channel::RecvError)));
        mock_source_change_rx
            .expect_try_recv()
            .returning(|| Err(RxError::TryRecv(crossbeam_channel::TryRecvError::Empty)));

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
            run_hyps: MaybeOwned::Owned(mock_run_hyps),
            source_change_rx: MaybeOwned::Owned(mock_source_change_rx),
            session_rx: MaybeOwned::Owned(mock_session_rx),
            log_rx: MaybeOwned::Owned(mock_log_rx)
        }
    }
}

impl<'a, TBridge: Bridge, _TRunHyps, _TRxSourceChange, _TRxSession, _TRxLog>
    UpdateApp<'a, TBridge, _TRunHyps, _TRxSourceChange, _TRxSession, _TRxLog>
{
    pub fn with_run_hyps<TRunHyps>(
        self,
        run_hyps: MaybeOwned<'a, TRunHyps>
    ) -> UpdateApp<'a, TBridge, TRunHyps, _TRxSourceChange, _TRxSession, _TRxLog>
    {
        UpdateApp {
            test_self: self.test_self,
            egui_context: self.egui_context,
            layout: self.layout,
            run_hyps,
            source_change_rx: self.source_change_rx,
            session_rx: self.session_rx,
            log_rx: self.log_rx
        }
    }

    pub fn with_source_change_rx<TRxSourceChange>(
        self,
        source_change_rx: MaybeOwned<'a, TRxSourceChange>
    ) -> UpdateApp<'a, TBridge, _TRunHyps, TRxSourceChange, _TRxSession, _TRxLog>
    {
        UpdateApp {
            test_self: self.test_self,
            egui_context: self.egui_context,
            layout: self.layout,
            run_hyps: self.run_hyps,
            source_change_rx,
            session_rx: self.session_rx,
            log_rx: self.log_rx
        }
    }

    pub fn with_session_rx<TRxSession>(
        self,
        session_rx: MaybeOwned<'a, TRxSession>
    ) -> UpdateApp<'a, TBridge, _TRunHyps, _TRxSourceChange, TRxSession, _TRxLog>
    {
        UpdateApp {
            test_self: self.test_self,
            egui_context: self.egui_context,
            layout: self.layout,
            run_hyps: self.run_hyps,
            source_change_rx: self.source_change_rx,
            session_rx,
            log_rx: self.log_rx
        }
    }

    pub fn with_log_rx<TRxLog>(
        self,
        log_rx: MaybeOwned<'a, TRxLog>
    ) -> UpdateApp<'a, TBridge, _TRunHyps, _TRxSourceChange, _TRxSession, TRxLog>
    {
        UpdateApp {
            test_self: self.test_self,
            egui_context: self.egui_context,
            layout: self.layout,
            run_hyps: self.run_hyps,
            source_change_rx: self.source_change_rx,
            session_rx: self.session_rx,
            log_rx
        }
    }
}

impl<'a, TBridge: Bridge, TRunHyps, TRxSourceChange, TRxSession, TRxLog>
    UpdateApp<'a, TBridge, TRunHyps, TRxSourceChange, TRxSession, TRxLog>
where
    TRunHyps: RunHypsBridge<TBridge>,
    TRxSourceChange: Rx<SourceChangeEvent>,
    TRxSession: Rx<HypSessionEvent<TBridge>>,
    TRxLog: Rx<LogMessage>
{
    pub fn call(&mut self)
    {
        self.test_self.update_app(
            self.egui_context,
            self.layout,
            self.run_hyps.as_ref(),
            self.source_change_rx.as_ref(),
            self.session_rx.as_ref(),
            self.log_rx.as_ref()
        );
    }
}
