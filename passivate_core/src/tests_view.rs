use std::thread;
use std::thread::JoinHandle;
use futures::channel::mpsc::{channel, Sender};
use futures::SinkExt;

pub struct TestsStatus {
    pub text: String
}

impl TestsStatus {
    pub fn new(text: &str) -> Self {
        TestsStatus { text: text.to_string() }
    }

    pub fn set_text(&mut self, text: &str) {
        self.text = text.to_string();
    }
}

impl Default for TestsStatus {
    fn default() -> TestsStatus {
        TestsStatus::new("")
    }
}

pub trait TestsView : Send {
    fn update(&mut self, status: TestsStatus);
}

pub struct AsyncTestsView {
    sender: Sender<TestsStatus>,
    thread: JoinHandle<i32>
}

impl AsyncTestsView {
    pub fn new(mut tests_view: Box<dyn TestsView>) -> Box<AsyncTestsView> {
        let (sender, mut receiver) = channel(1);
        let thread = thread::spawn(move  || {
            while let Ok(status) = receiver.try_next(){
                tests_view.update(status.unwrap());
            }

            0
        });

        Box::new(Self { sender, thread } )
    }
}

impl TestsView for AsyncTestsView {
    fn update(&mut self, status: TestsStatus) {
        futures::executor::block_on(async {
            self.sender.send(status).await.unwrap();
        });
    }
}