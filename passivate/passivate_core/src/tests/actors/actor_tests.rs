use std::{sync::mpsc::{channel, Sender}, thread::{self, JoinHandle}};

#[derive(Default)]
struct ExampleHandler {
    total: i32
}

impl ExampleHandler {
    pub fn get_total(&self) -> i32 {
        self.total
    }
}

impl Handler<i32> for ExampleHandler {
    fn handle(&mut self, event: i32) {
        self.total += event;
    }
}

enum ActorEvent<T: Send + 'static> {
    Custom(T),
    Exit
}

struct Actor<T: Send + Clone + 'static, THandler: Handler<T>> {
    api: Api<T>,
    thread: Option<JoinHandle<THandler>>
}

trait Handler<T: Send + 'static> : Send + 'static {
    fn handle(&mut self, event: T);
}

#[derive(Clone)]
struct Api<T: Send + Clone + 'static> {
    sender: Sender<ActorEvent<T>>
}

impl<T: Send + Clone + 'static> Api<T> {
    pub fn send(&self, event: T) {
        self.sender.send(ActorEvent::Custom(event)).unwrap();
    }

    fn exit(&self) {
        self.sender.send(ActorEvent::Exit).unwrap();
    }
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

        let api = Api { sender };
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

#[test]
pub fn bla() {
    let handler = ExampleHandler::default();
    let mut actor = Actor::new(handler);

    let api = actor.api();

    api.send(16);
    api.send(32);

    let handler = actor.stop();

    assert_eq!(48, handler.get_total());
}