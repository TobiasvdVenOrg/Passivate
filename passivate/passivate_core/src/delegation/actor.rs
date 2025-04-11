use std::{error::Error, fmt::Display, sync::{atomic::{AtomicBool, Ordering}, mpsc::{channel, Sender}, Arc}, thread::{self, JoinHandle}};

use super::{actor_event::ActorEvent, ActorApi, Give, Handler, Loan};

#[derive(Default, Clone)]
pub struct Cancellation {
    flag: Arc<AtomicBool>
}

#[derive(Debug, Clone, PartialEq)]
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

pub struct Actor<T: Send + 'static, THandler: Handler<T>> {
    sender: Sender<ActorEvent<T>>,
    thread: Option<JoinHandle<THandler>>
}

impl<T: Send + 'static, THandler: Handler<T>> Actor<T, THandler> {
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

        Self { sender, thread }
    }

    pub fn give(&self) -> impl Give<T> {
        ActorApi::new(self.sender.clone())
    }

    pub fn loan(&self) -> impl Loan<T> {
        ActorApi::new(self.sender.clone())
    }

    pub fn stop(&mut self) -> THandler {
        self.sender.send(ActorEvent::Exit).expect("failed to stop actor!");
        let thread = self.thread.take().expect("failed to acquire actor thread handle!");

        thread.join().expect("failed to join actor thread!")
    }
}