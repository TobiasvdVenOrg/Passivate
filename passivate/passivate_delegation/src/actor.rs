use std::error::Error;
use std::fmt::Display;
use std::marker::PhantomData;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::thread::{self, JoinHandle};

use crossbeam_channel::Receiver;

use super::{ActorEvent, ActorTx, Handler};

#[derive(Default, Clone)]
pub struct Cancellation
{
    flag: Arc<AtomicBool>
}

#[derive(Debug, Clone, PartialEq)]
pub struct Cancelled;

impl Cancellation
{
    pub fn is_cancelled(&self) -> bool
    {
        self.flag.load(Ordering::SeqCst)
    }

    pub fn cancel(&mut self)
    {
        self.flag.store(true, Ordering::SeqCst);
    }

    pub fn check(&self) -> Result<(), Cancelled>
    {
        if self.is_cancelled()
        {
            return Err(Cancelled);
        }

        Ok(())
    }
}

impl Error for Cancelled {}

impl Display for Cancelled
{
    fn fmt(&self, _f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
    {
        Ok(())
    }
}

pub struct Actor<T: Send + 'static, THandler: Handler<T>>
{
    thread: Option<JoinHandle<THandler>>,
    _phantom: PhantomData<T>
}

impl<T: Send + 'static, THandler: Handler<T>> Actor<T, THandler>
{
    /// An actor will asynchronously listen for events posted on the 'tx' returned by 'new'.
    /// Similar to channels, all tx instances (including clones) must be dropped for the actor the shut down gracefully.
    /// Dropping an actor while an associated tx instance is still alive may cause a deadlock.
    /// The most reliable way to prevent this from happening is to immediately move the tx instance after acquisition.
    /// This will guarantee that it is dropped before the actor reaches the end of the scope, for example:
    /// ```
    /// # use passivate_delegation::{Actor, ActorTx, Cancellation, Handler, Tx};
    /// #
    /// # struct ExampleHandler;
    /// #
    /// # impl Handler<i32> for ExampleHandler {
    /// #     fn handle(&mut self, event: i32, _cancellation: Cancellation) {
    /// #
    /// #     }
    /// # }
    /// #
    /// # fn do_something(tx: ActorTx<i32>) {
    /// #     tx.send(16, Cancellation::default());
    /// # }
    /// #
    /// {
    ///     let handler = ExampleHandler {};
    ///     let (actor, tx) = Actor::new(handler);
    ///
    ///     // Move 'tx' here
    ///     do_something(tx);
    ///
    ///     // 'tx' is guaranteed to be dropped when 'actor' is dropped at the end of the scope
    /// }
    /// ```
    pub fn new(handler: THandler) -> (Actor<T, THandler>, ActorTx<T>)
    {
        let (tx, rx) = crossbeam_channel::unbounded::<ActorEvent<T>>();

        let actor_tx = ActorTx::new(tx);
        
        let actor = Self::with_rx(handler, rx);

        (actor, actor_tx)
    }

    pub fn with_rx(mut handler: THandler, rx: Receiver<ActorEvent<T>>) -> Actor<T, THandler>
    {
        let thread = Some(thread::spawn(move || {
            while let Ok(event) = rx.recv()
            {
                handler.handle(event.event, event.cancellation);
            }

            handler
        }));

        Self { thread, _phantom: PhantomData {} }
    }

    pub fn new_2(mut handler: THandler) -> (Actor<T, THandler>, ActorTx<T>, ActorTx<T>)
    {
        let (tx, rx) = crossbeam_channel::unbounded::<ActorEvent<T>>();

        let thread = Some(thread::spawn(move || {
            while let Ok(event) = rx.recv()
            {
                handler.handle(event.event, event.cancellation);
            }

            handler
        }));

        let actor_tx1 = ActorTx::new(tx.clone());
        let actor_tx2 = ActorTx::new(tx);
        let actor = Self { thread, _phantom: PhantomData {} };
        (actor, actor_tx1, actor_tx2)
    }

    pub fn new_3(mut handler: THandler) -> (Actor<T, THandler>, ActorTx<T>, ActorTx<T>, ActorTx<T>)
    {
        let (tx, rx) = crossbeam_channel::unbounded::<ActorEvent<T>>();

        let thread = Some(thread::spawn(move || {
            while let Ok(event) = rx.recv()
            {
                handler.handle(event.event, event.cancellation);
            }

            handler
        }));

        let actor_tx1 = ActorTx::new(tx.clone());
        let actor_tx2 = ActorTx::new(tx.clone());
        let actor_tx3 = ActorTx::new(tx);
        let actor = Self { thread, _phantom: PhantomData {} };
        (actor, actor_tx1, actor_tx2, actor_tx3)
    }

    pub fn into_inner(&mut self) -> THandler
    {
        let thread = self.thread.take().expect("failed to acquire actor thread handle!");

        thread.join().expect("failed to join actor thread!")
    }
}

impl<T: Send + 'static, THandler: Handler<T>> Drop for Actor<T, THandler>
{
    fn drop(&mut self)
    {
        if let Some(thread) = self.thread.take()
        {
            let _handler = thread.join().expect("failed to join actor thread!");
        }
    }
}

pub struct Actor2<TEvent: Send + 'static, TReturn: Send + 'static>
{
    tx: ActorTx<TEvent>,
    handle: Option<JoinHandle<TReturn>>
}

impl<TEvent, TReturn> Actor2<TEvent, TReturn>
where
    TEvent: Send + 'static,
    TReturn: Send + 'static
{
    pub fn new<THandler>(handler: THandler) -> Self
    where
        THandler: FnOnce(Receiver<ActorEvent<TEvent>>) -> TReturn + Send + 'static
    {
        let (tx, rx) = crossbeam_channel::unbounded();

        let handle = thread::spawn(move || handler(rx));

        let actor_tx = ActorTx::new(tx);

        Actor2 { tx: actor_tx, handle: Some(handle) }
    }

    pub fn send(&self, event: TEvent)
    {
        self.tx.send(event, Cancellation::default());
    }

    pub fn send_cancellable(&self, event: TEvent, cancellation: Cancellation)
    {
        self.tx.send(event, cancellation);
    }

    pub fn into_inner(mut self) -> TReturn
    {
        let handle = self.handle.take().expect("failed to acquire actor thread handle");

        drop(self);
        
        handle.join().expect("actor failed to join.")
    }
}
