use std::{sync::mpsc::channel, thread::{self, JoinHandle}};

use super::{actor_event::ActorEvent, Api, Handler};

pub struct Actor<T: Send + Clone + 'static, THandler: Handler<T>> {
    api: Api<T>,
    thread: Option<JoinHandle<THandler>>
}

impl<T: Send + Clone + 'static, THandler: Handler<T>> Actor<T, THandler> {
    pub fn new(mut handler: THandler) -> Self {
        let (sender, receiver) = channel();

        let thread = Some(thread::spawn(move || {
            while let Ok(event) = receiver.recv() {
                match event {
                    ActorEvent::Custom(custom) => handler.handle(custom),
                    ActorEvent::Exit => break,
                }
            }

            handler
        }));

        let api = Api::new(sender);
        Self { api, thread }
    }

    pub fn api(&self) -> Api<T> {
        self.api.clone()
    }

    pub fn stop(&mut self) -> THandler {
        self.api.exit();
        let thread = self.thread.take().unwrap();

        thread.join().unwrap()
    }
}