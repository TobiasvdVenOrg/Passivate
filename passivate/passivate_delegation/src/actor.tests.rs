use std::time::Duration;

use crossbeam_channel::Receiver;

use crate::{Actor, Actor2, ActorEvent, ActorTx, Cancellation, Handler};

struct LoopingHandler;

impl Handler<i32> for LoopingHandler
{
    fn handle(&mut self, _event: i32, cancellation: Cancellation)
    {
        loop
        {
            std::thread::sleep(Duration::from_millis(100));

            if cancellation.is_cancelled()
            {
                return;
            }
        }
    }
}

struct ExampleEvent
{
    input: i32
}

struct ExampleCollaborator
{
    value: i32
}

impl ExampleCollaborator
{
    pub fn set_value(&mut self, value: i32)
    {
        self.value = value;
    }
}

trait ExampleTrait
{
    fn get_something(&self) -> i32;
}

struct ExampleImpl;

impl ExampleTrait for ExampleImpl
{
    fn get_something(&self) -> i32
    {
        10
    }
}

#[test]
pub fn actor_handles_event_and_returns()
{
    let collaborator = ExampleCollaborator { value: 0 };
    let thing = ExampleImpl;

    let actor = Actor2::new(move |event| actor_thread(event, collaborator, thing));

    actor.send(ExampleEvent { input: 32 });

    let result = actor.into_inner();

    assert_eq!(42, result);
}

fn actor_thread(rx: Receiver<ActorEvent<ExampleEvent>>, mut collaborator: ExampleCollaborator, thing: impl ExampleTrait) -> i32
{
    while let Ok(event) = rx.recv()
    {
        handle(event.event, &mut collaborator);
    }

    collaborator.value + thing.get_something()
}

fn handle(event: ExampleEvent, collaborator: &mut ExampleCollaborator)
{
    collaborator.set_value(event.input);
}

#[test]
pub fn actor_handle_can_be_cancelled()
{
    let handler = LoopingHandler;
    let mut cancellation = Cancellation::default();
    let (_actor, tx) = Actor::new(handler);

    send(tx, cancellation.clone());

    cancellation.cancel();
}

fn send(tx: ActorTx<i32>, cancellation: Cancellation)
{
    tx.send(64, cancellation);
}
