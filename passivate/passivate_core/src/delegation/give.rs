use super::ActorApi;


#[mockall::automock]
pub trait Give<T: Send + 'static> : Send {
    fn send(&self, event: T);
}

impl<T: Send + 'static> Give<T> for std::sync::mpsc::Sender<T> {
    fn send(&self, event: T) {
        self.send(event).expect("'Give' failed, receiver is invalid!");
    }
}

impl<T: Send + 'static> Give<T> for crossbeam_channel::Sender<T> {
    fn send(&self, event: T) {
        self.send(event).expect("'Give' failed, crossbeam channel is invalid!");
    }
}

impl<T: Send + 'static> Give<T> for ActorApi<T> {
    fn send(&self, event: T) {
        self.send(event).expect("'Give' failed, actor is invalid!");
    }
}

pub fn stub_give<T: Send + 'static>() -> Box<dyn Give<T>> {
    let mut mock = MockGive::new();
    mock.expect_send().returning(|_| { });

    Box::new(mock)
}
