use crate::configuration::{ConfigurationChangeEvent, ConfigurationHandler};
use crate::actors::{Actor, Cancellation, Handler};
use crate::test_helpers::fakes::change_event_handler_fakes;


#[test]
pub fn coverage_enabled() {
    let change_event_handler = change_event_handler_fakes::stub();
    let actor = Actor::new(change_event_handler);
    let mut handler = ConfigurationHandler::new(actor.api());
    
    assert!(!handler.configuration().coverage_enabled);

    let change = ConfigurationChangeEvent::Coverage(true);
    handler.handle(change, Cancellation::default());

    assert!(handler.configuration().coverage_enabled);
}
