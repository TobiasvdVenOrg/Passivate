use std::thread;
use std::thread::JoinHandle;
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
            self.sender.send(event).await.unwrap();
        });
    }
}

impl AsyncChangeEventHandler {
    pub fn new(mut handler: Box<dyn ChangeEventHandler>) -> Box<Self> {
        let (sender, mut receiver) = channel(1);
        let thread = thread::spawn(move  || {
            while let Ok(change_event) = receiver.try_next(){
                handler.handle_event(change_event.unwrap());
            }

            0
        });

        Box::new(Self { sender, thread } )
    }
}
