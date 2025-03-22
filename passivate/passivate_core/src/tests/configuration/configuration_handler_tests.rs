use std::sync::mpsc::channel;

use crate::configuration::{ConfigurationChangeEvent, ConfigurationHandler};
use crate::actors::{Cancellation, Handler};
use crate::test_helpers::fakes::stub_actor_api;


#[test]
pub fn configuration_change_is_broadcasted() {
    let (configuration_sender, configuration_receiver) = channel();
    let mut handler = ConfigurationHandler::new(stub_actor_api(), configuration_sender);

    handler.handle(ConfigurationChangeEvent::Coverage(true), Cancellation::default());

    let broadcast = configuration_receiver.try_iter().last().unwrap();

    assert!(broadcast.coverage_enabled);
}