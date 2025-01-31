use notify::{RecommendedWatcher, Config, RecursiveMode, Watcher};
use std::path::Path;
use crate::error::{PassivateError, Result};
use crate::change_events::{ChangeEvent, ChangeEventHandler};

pub struct NotifyChangeEvents {
    watcher: RecommendedWatcher
}

impl NotifyChangeEvents {
    pub fn new(path: &str, mut handler: Box<dyn ChangeEventHandler>) -> Result<NotifyChangeEvents> {
        let mut watcher = RecommendedWatcher::new(move |res| {
            match res {
                Ok(event) => {
                    println!("{:?}", event);
                    let change_event = ChangeEvent { };
                    handler.handle_event(change_event);
                }
                Err(error) => {
                    println!("{:?}", error);
                }
            }
        }, Config::default()).expect("Unable to create watcher.");

        let watch_result = watcher.watch(Path::new(&path), RecursiveMode::Recursive);

        match watch_result {
            Err(error) => {
                Err(PassivateError::notify(error, path))
            }
            Ok(_) => {
                Ok(NotifyChangeEvents { watcher })
            }
        }
    }
}
