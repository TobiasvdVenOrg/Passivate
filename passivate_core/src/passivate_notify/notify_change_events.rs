use std::collections::HashMap;
use std::fs;
use std::hash::Hash;
use notify::{ReadDirectoryChangesWatcher, RecommendedWatcher, RecursiveMode, Watcher};
use notify::Config as NotifyConfig;
use notify::Event as NotifyEvent;
use notify::Result as NotifyResult;
use std::path::{Path, PathBuf};
use std::time::SystemTime;
use crate::change_events::{ChangeEvent, HandleChangeEvent};
use crate::passivate_notify::NotifyChangeEventsError;

pub struct NotifyChangeEvents {
    watcher: RecommendedWatcher
}

impl NotifyChangeEvents {
    pub fn new(path: &str, mut event_handler: Box<dyn HandleChangeEvent>) -> Result<NotifyChangeEvents, NotifyChangeEventsError> {
        let mut modification_cache: HashMap<PathBuf, SystemTime> = HashMap::new();

        let config = NotifyConfig::default().with_compare_contents(true);

        let mut watcher = RecommendedWatcher::new(move |event: NotifyResult<NotifyEvent>| {
            match event {
                Ok(event) => {
                    for path in &event.paths {
                        let extension = path.extension();

                        if let Some(extension) = extension {
                            if extension == "rs" {
                                println!("Checking...");
                                if let Ok(metadata) = fs::metadata(path) {
                                    if let Ok(modified) = metadata.modified() {
                                        if let Some(last_modification) = modification_cache.get(path.as_path()) {
                                            println!("Last modified was: {:?}", last_modification);

                                            if &modified > last_modification {
                                                println!("Running... {:?}", modified);
                                                println!("{:?}", event);
                                                let change_event = ChangeEvent { };
                                                event_handler.handle_event(change_event);
                                            } else {
                                                println!("Ignoring...");
                                            }
                                        } else {
                                            println!("Not in cache! {:?}", path);
                                            let change_event = ChangeEvent { };
                                            event_handler.handle_event(change_event);
                                        }

                                        modification_cache.insert(path.clone(), modified);
                                    }
                                }

                            }
                        }
                    }
                }
                Err(error) => {
                    println!("{:?}", error);
                }
            }
        }, config);

        match watcher {
            Ok(mut watcher) => {
                watcher = Self::start_watcher(watcher, path)?;

                Ok(NotifyChangeEvents { watcher })
            }
            Err(notify_error) => {
                Err(NotifyChangeEventsError::invalid_path(path, notify_error))
            }
        }

    }

    fn start_watcher(mut watcher: ReadDirectoryChangesWatcher, path: &str) -> Result<ReadDirectoryChangesWatcher, NotifyChangeEventsError> {
        let watch_result = watcher.watch(Path::new(&path), RecursiveMode::Recursive);

        match watch_result {
            Ok(_) => Ok(watcher),
            Err(notify_error) => Err(NotifyChangeEventsError::invalid_path(path, notify_error))
        }
    }
}
