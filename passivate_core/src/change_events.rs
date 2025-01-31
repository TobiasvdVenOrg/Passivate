use std::thread;
use std::thread::JoinHandle;
use std::time::Duration;
use futures::channel::mpsc::channel;
use futures::SinkExt;

pub struct ChangeEvent {

}

pub trait ChangeEventHandler : Send {
    fn handle_event(&mut self, event: ChangeEvent);
}

pub struct AsyncChangeEventHandler {
    sender: futures::channel::mpsc::Sender<ChangeEvent>,
    thread: JoinHandle<i32>
}

impl ChangeEventHandler for AsyncChangeEventHandler {
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

impl AsyncChangeEventHandler {
    pub fn new(mut handler: Box<dyn ChangeEventHandler>) -> Box<Self> {
        let (sender, mut receiver) = channel(1);
        let thread = thread::spawn(move  || {
            loop {
                if let Ok(Some(change_event)) = receiver.try_next() {
                    handler.handle_event(change_event);
                }

                thread::sleep(Duration::from_millis(1000));
            }

            0
        });

        Box::new(Self { sender, thread } )
    }
}
