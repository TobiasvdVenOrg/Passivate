use std::{error::Error, fmt::Display, marker::PhantomData, sync::{atomic::{AtomicBool, Ordering}, Arc}, thread::{self, JoinHandle}};

use super::{ActorEvent, ActorTx, Handler};

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
    thread: Option<JoinHandle<THandler>>,
    _phantom: PhantomData<T>
}

impl<T: Send + 'static, THandler: Handler<T>> Actor<T, THandler> {
    pub fn new(mut handler: THandler) -> (ActorTx<T>, Actor<T, THandler>) {
        let (tx, rx) = crossbeam_channel::unbounded::<ActorEvent<T>>();

        let thread = Some(thread::spawn(move || {
            while let Ok(event) = rx.recv() {
                handler.handle(event.event, event.cancellation);
            }

            handler
        }));

        let actor_tx = ActorTx::new(tx);
        let actor = Self { thread, _phantom: PhantomData { } };
        ( actor_tx, actor )
    }

    pub fn into_inner(&mut self) -> THandler {
        let thread = self.thread.take().expect("failed to acquire actor thread handle!");

        thread.join().expect("failed to join actor thread!")
    }
}
