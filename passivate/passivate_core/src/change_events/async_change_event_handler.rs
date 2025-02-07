use std::sync::mpsc::{Receiver};
use std::thread;
use std::time::Duration;
use crate::change_events::change_event::ChangeEvent;
use crate::change_events::HandleChangeEvent;

pub struct AsyncChangeEventHandler {
}

impl AsyncChangeEventHandler {
    pub fn new(mut handler: Box<dyn HandleChangeEvent>, receiver: Receiver<ChangeEvent>) -> AsyncChangeEventHandler {
        let _ = thread::spawn(move  || {
            loop {
                if let Ok(change_event) = receiver.try_recv() {
                    handler.handle_event(change_event);
                }

                thread::sleep(Duration::from_millis(1000));
            }
        });

        AsyncChangeEventHandler { }
    }
}

