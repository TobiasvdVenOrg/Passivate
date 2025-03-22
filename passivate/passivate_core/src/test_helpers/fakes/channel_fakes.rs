use std::sync::mpsc::{channel, Sender};



pub fn stub_sender<T>() -> Sender<T> {
    let (sender, _receiver) = channel();

    sender
}