
use notify::*;
use std::path::Path;
use crate::change_events::{ChangeEvent, ChangeEventHandler};

pub struct NotifyChangeEvents {
    watcher: RecommendedWatcher
}

impl NotifyChangeEvents {
    pub fn new(path: &str, mut handler: Box<dyn ChangeEventHandler>) -> Self {
        let mut watcher = RecommendedWatcher::new(move |res| {
            futures::executor::block_on(async {
                match res {
                    Ok(_event) => {
                        let change_event = ChangeEvent { };
                        handler.handle_event(change_event);
                    }
                    Err(error) => {
                        println!("{:?}", error);
                    }
                }
            })
        }, Config::default()).expect("Unable to create watcher.");

        let _ = watcher.watch(Path::new(&path), RecursiveMode::Recursive).expect("Unable to start watching.");

        Self { watcher }
    }
}
