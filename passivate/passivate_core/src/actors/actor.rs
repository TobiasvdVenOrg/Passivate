use std::{error::Error, fmt::Display, sync::{atomic::{AtomicBool, Ordering}, mpsc::channel, Arc}, thread::{self, JoinHandle}};

use super::{actor_event::ActorEvent, ActorApi, Handler};

#[derive(Default, Clone)]
pub struct Cancellation {
    flag: Arc<AtomicBool>
}

#[derive(Debug, Clone)]
pub struct Cancelled;

impl Cancellation {
    pub fn is_cancelled(&self) -> bool {
        self.flag.load(Ordering::SeqCst)
    }

    pub fn cancel(&mut self) {
        self.flag.store(true, Ordering::SeqCst);
    }

    pub fn check(&self) -> Result<(), Cancelled> {
        if self.is_cancelled() {
            return Err(Cancelled);
        }

        Ok(())
    }
}

impl Error for Cancelled {
    
}

impl Display for Cancelled {
    fn fmt(&self, _f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Ok(())
    }
}

pub struct Actor<T: Send + Clone + 'static, THandler: Handler<T>> {
    api: ActorApi<T>,
    thread: Option<JoinHandle<THandler>>
}

impl<T: Send + Clone + 'static, THandler: Handler<T>> Actor<T, THandler> {
    pub fn new(mut handler: THandler) -> Self {
        let (sender, receiver) = channel();

        let thread = Some(thread::spawn(move || {
            while let Ok(event) = receiver.recv() {
                match event {
                    ActorEvent::Custom(custom) => handler.handle(custom, Cancellation::default()),
                    ActorEvent::Cancellable(cancellable) => handler.handle(cancellable.event, cancellable.cancellation),
                    ActorEvent::Exit => break,
                };
            }

            handler
        }));

        let api = ActorApi::new(sender);
        Self { api, thread }
    }

    pub fn api(&self) -> ActorApi<T> {
        self.api.clone()
    }

    pub fn stop(&mut self) -> THandler {
        self.api.exit();
        let thread = self.thread.take().unwrap();

        thread.join().unwrap()
    }
}