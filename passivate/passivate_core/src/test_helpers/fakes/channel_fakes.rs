use std::sync::mpsc::{channel, Receiver};

pub fn stub_receiver<T>() -> Receiver<T> {
    let (_sender, receiver) = channel();

    receiver
}

pub fn stub_crossbeam_receiver<T>() -> crossbeam_channel::Receiver<T> {
    let (_sender, receiver) = crossbeam_channel::unbounded();

    receiver
}
