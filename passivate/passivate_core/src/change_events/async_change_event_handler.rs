use std::sync::mpsc::{Receiver};
use std::thread;
use crate::change_events::change_event::ChangeEvent;
use crate::change_events::HandleChangeEvent;

pub struct AsyncChangeEventHandler {
}

impl AsyncChangeEventHandler {
    pub fn new(mut handler: Box<dyn HandleChangeEvent>, receiver: Receiver<ChangeEvent>) -> AsyncChangeEventHandler {
        let _ = thread::spawn(move  || {
            loop {
                if let Ok(change_event) = receiver.recv() {
                    handler.handle_event(change_event);
                }
            }
        });

        AsyncChangeEventHandler { }
    }
}

