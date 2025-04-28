use crate::configuration::{ConfigurationChangeEvent, ConfigurationHandler};
use passivate_delegation::{channel, Cancellation, Handler, Tx};

#[test]
pub fn configuration_change_is_broadcasted() {
    let (configuration_sender, configuration_receiver) = channel();
    let mut handler = ConfigurationHandler::new(Tx::stub(), configuration_sender);

    handler.handle(ConfigurationChangeEvent::Coverage(true), Cancellation::default());

    let broadcast = configuration_receiver.try_recv().unwrap();

    assert!(broadcast.new.coverage_enabled);
}