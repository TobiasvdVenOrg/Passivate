use std::time::Duration;

use crossbeam_channel::Receiver;

use crate::{Actor2, ActorEvent, Cancellation};

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

    let actor = Actor2::new(move |rx| actor_thread(rx, collaborator, thing));

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
pub fn actor_event_is_cancellable()
{
    let actor = Actor2::new(actor_thread_infinite);

    let mut cancellation = Cancellation::default();

    actor.send_cancellable(ExampleEvent { input: 32 }, cancellation.clone());
    cancellation.cancel();

    let result = actor.into_inner();

    assert_eq!(0, result);
}

fn actor_thread_infinite(rx: Receiver<ActorEvent<ExampleEvent>>) -> i32
{
    while let Ok(event) = rx.recv()
    {
        handle_infinite(event.event, event.cancellation);
    }

    0
}

fn handle_infinite(_event: ExampleEvent, cancellation: Cancellation)
{
    loop
    {
        std::thread::sleep(Duration::from_millis(100));

        if cancellation.is_cancelled()
        {
            break;
        }
    }
}
