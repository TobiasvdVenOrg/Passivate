use crate::configuration::{ConfigurationChangeEvent, ConfigurationHandler};
use crate::delegation::{Cancellation, Handler};
use crate::test_helpers::fakes::actor_fakes::stub_actor_api;


#[test]
pub fn configuration_change_is_broadcasted() {
    let (configuration_sender, configuration_receiver) = crossbeam_channel::unbounded();
    let mut handler = ConfigurationHandler::new(stub_actor_api(), configuration_sender);

    handler.handle(ConfigurationChangeEvent::Coverage(true), Cancellation::default());

    let broadcast = configuration_receiver.try_iter().last().unwrap();

    assert!(broadcast.new.coverage_enabled);
}