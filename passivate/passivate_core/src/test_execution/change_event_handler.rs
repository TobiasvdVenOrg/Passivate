use crate::actors::{ActorApi, Cancellation, Handler};
use crate::change_events::ChangeEvent;
use crate::cross_cutting::Log;

pub struct ChangeEventHandler {
    test_run_handler: ActorApi<ChangeEvent>,
    log: Box<dyn Log + Send>,

    cancellation: Cancellation
}

impl ChangeEventHandler {
    pub fn new(test_run_handler: ActorApi<ChangeEvent>, log: Box<dyn Log + Send>) -> Self {
        Self { test_run_handler, log, cancellation: Cancellation::default() }
    }
}

impl Handler<ChangeEvent> for ChangeEventHandler {
    fn handle(&mut self, event: ChangeEvent, _cancellation: Cancellation) {
        self.log.info("Handling it!");

        self.cancellation.cancel();
        self.cancellation = Cancellation::default();

        self.test_run_handler.send_cancellable(event, self.cancellation.clone());
        
        self.log.info("Done sending it!");
    }
}
