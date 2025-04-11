use crate::configuration::{ConfigurationChangeEvent, ConfigurationHandler};
use crate::delegation::{stub_give, Cancellation, Handler};

#[test]
pub fn configuration_change_is_broadcasted() {
    let (configuration_sender, configuration_receiver) = crossbeam_channel::unbounded();
    let mut handler = ConfigurationHandler::new(stub_give(), Box::new(configuration_sender));

    handler.handle(ConfigurationChangeEvent::Coverage(true), Cancellation::default());

    let broadcast = configuration_receiver.try_iter().last().unwrap();

    assert!(broadcast.new.coverage_enabled);
}