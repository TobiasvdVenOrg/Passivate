use std::thread::{self, JoinHandle};

use bon::Builder;
use passivate_configuration::configuration_manager::ConfigurationManager;
use passivate_coverage::compute_coverage::ComputeCoverage;
use passivate_coverage::coverage_status::CoverageStatus;
use passivate_delegation::{CancellableMessage, Cancellation, Rx, Tx};
use passivate_hyp_names::hyp_id::HypId;
use passivate_model_core::hyp_run_trigger::HypRunTrigger;
use passivate_model_core::hyp_session_event::HypSessionEvent;

use crate::hyp_runner::HypRunner;

pub fn test_run_thread(rx: Rx<CancellableMessage<HypRunTrigger>>, mut handler: HypRunHandler) -> JoinHandle<HypRunHandler>
{
    thread::spawn(move || {
        while let Ok(event) = rx.recv()
        {
            handler.handle(event.message, event.cancellation);
        }

        handler
    })
}

#[derive(Builder)]
pub struct HypRunHandler
{
    runner: HypRunner,
    coverage: Box<dyn ComputeCoverage + Send>,
    hyp_run_tx: Tx<HypSessionEvent>,
    coverage_tx: Tx<CoverageStatus>,
    configuration: ConfigurationManager,
    pinned_hyp: Option<HypId>
}

impl HypRunHandler
{
    pub fn handle(&mut self, event: HypRunTrigger, cancellation: Cancellation)
    {
        match event
        {
            HypRunTrigger::DefaultRun => self.run_hyps(cancellation.clone()),
            HypRunTrigger::PinHyp { id } =>
            {
                self.pinned_hyp = Some(id);
                self.run_hyps(cancellation.clone());
            }
            HypRunTrigger::ClearPinnedHyps =>
            {
                self.pinned_hyp = None;
                self.run_hyps(cancellation.clone());
            }
            HypRunTrigger::Hyp { id, update_snapshots } => self.run_hyp(&id, update_snapshots, cancellation.clone())
        }
    }

    fn run_hyps(&mut self, cancellation: Cancellation)
    {
        if let Some(pinned_hyp) = self.pinned_hyp.clone()
        {
            let update_snapshots = false;
            self.run_hyp(&pinned_hyp, update_snapshots, cancellation.clone());
            return;
        }

        let coverage_enabled = self.coverage_enabled();

        if coverage_enabled
        {
            self.coverage_tx.send(CoverageStatus::Preparing);
        }

        if let Err(clean_error) = self.coverage.clean_coverage_output()
        {
            log::error!("error cleaning coverage output: {:?}", clean_error);
        }

        if cancellation.is_cancelled()
        {
            return;
        }

        let test_output = self
            .runner
            .run_hyps(coverage_enabled, cancellation.clone(), &mut self.hyp_run_tx, Vec::new());

        if cancellation.is_cancelled()
        {
            return;
        }

        match test_output
        {
            Ok(_) =>
            {
                if self.coverage_enabled()
                {
                    log::info!("Coverage enabled, computing...");
                    self.compute_coverage(cancellation.clone());
                }
                else
                {
                    log::info!("Coverage disabled.");
                }
            }
            Err(test_error) =>
            {
                todo!()
            }
        };
    }

    fn compute_coverage(&mut self, cancellation: Cancellation)
    {
        self.coverage_tx.send(CoverageStatus::Running);

        let coverage_status = self.coverage.compute_coverage(cancellation.clone());

        log::info!("Coverage completed.");

        match coverage_status
        {
            Ok(coverage_status) => self.coverage_tx.send(coverage_status),
            Err(coverage_error) => self.coverage_tx.send(CoverageStatus::Error(coverage_error.to_string()))
        }
    }

    fn run_hyp(&mut self, id: &HypId, update_snapshots: bool, cancellation: Cancellation)
    {
        let result = self.runner.run_hyp(id, update_snapshots, cancellation, &mut self.hyp_run_tx);

        if let Err(error) = result
        {
            todo!()
        }
    }

    pub fn coverage_enabled(&self) -> bool
    {
        self.configuration.get(|c| c.coverage_enabled)
    }
}
