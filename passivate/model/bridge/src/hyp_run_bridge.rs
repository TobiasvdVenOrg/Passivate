use passivate_configuration::configuration::PassivateConfiguration;
use passivate_configuration::default_paths::DefaultPaths;
use passivate_delegation::tx_rx::Tx;

use crate::bridge::Bridge;
use crate::hyp_run_request::HypRunRequest;

/// Interface from a session state to start test runs.
#[mockall::automock]
pub trait RunHypsBridge<TBridge: Bridge>
{
    fn run_all(&self, configuration: PassivateConfiguration, paths: DefaultPaths);
    fn run_single(&self, hyp: TBridge::Id, configuration: PassivateConfiguration, paths: DefaultPaths);
}

impl<TTx, TBridge> RunHypsBridge<TBridge> for TTx
where
    TBridge: Bridge,
    TTx: Tx<HypRunRequest<TBridge>>
{
    fn run_all(&self, configuration: PassivateConfiguration, paths: DefaultPaths)
    {
        self.send(HypRunRequest::all(configuration, paths));
    }

    fn run_single(&self, hyp_id: TBridge::Id, configuration: PassivateConfiguration, paths: DefaultPaths)
    {
        self.send(HypRunRequest::single(hyp_id, configuration, paths));
    }
}
