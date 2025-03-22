use std::sync::mpsc::{channel, Receiver, Sender};



pub fn stub_sender<T>() -> Sender<T> {
    let (sender, _receiver) = channel();

    sender
}

pub fn stub_receiver<T>() -> Receiver<T> {
    let (_sender, receiver) = channel();

    receiver
}