use std::thread;
use std::time::Duration;
use futures::channel::mpsc::channel;
use futures::SinkExt;
use crate::change_events::change_event::ChangeEvent;
use crate::change_events::handle_change_event::HandleChangeEvent;

pub struct AsyncChangeEventHandler {
    sender: futures::channel::mpsc::Sender<ChangeEvent>
}

impl AsyncChangeEventHandler {
    pub fn new(mut handler: Box<dyn HandleChangeEvent>) -> Box<Self> {
        let (sender, mut receiver) = channel(1);
        let _ = thread::spawn(move  || {
            loop {
                if let Ok(Some(change_event)) = receiver.try_next() {
                    handler.handle_event(change_event);
                }

                thread::sleep(Duration::from_millis(1000));
            }
        });

        Box::new(Self { sender } )
    }
}

impl HandleChangeEvent for AsyncChangeEventHandler {
    fn handle_event(&mut self, event: ChangeEvent) {
        futures::executor::block_on(async {
            let r = self.sender.send(event).await;

            match r {
                Ok(_) => {},
                Err(_) => {
                    println!("Error sending change event");
                }
            }
        });
    }
}
