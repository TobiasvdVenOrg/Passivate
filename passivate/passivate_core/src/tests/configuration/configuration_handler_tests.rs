use crate::{actors::{mock_handler, Actor, Cancellation, Handler}, configuration::{ConfigurationEvent, ConfigurationHandler}};


#[test]
pub fn coverage_enabled() {
    let mock_handler = mock_handler();
    let actor = Actor::new(mock_handler);
    let mut handler = ConfigurationHandler::new(actor.api());
    
    assert!(!handler.configuration().coverage_enabled);

    let change = ConfigurationEvent::Coverage(true);
    handler.handle(change, Cancellation::default());

    assert!(handler.configuration().coverage_enabled);
}
