use std::collections::HashMap;
use std::fs;
use notify::{RecommendedWatcher, RecursiveMode, Watcher};
use notify::Config as NotifyConfig;
use notify::Event as NotifyEvent;
use notify::Result as NotifyResult;
use std::path::{Path, PathBuf};
use std::time::SystemTime;
use crate::change_events::{ChangeEvent, HandleChangeEvent};
use crate::error::{PassivateError, Result};

pub struct NotifyChangeEvents {
    watcher: RecommendedWatcher
}

impl NotifyChangeEvents {
    pub fn new(path: &str, mut handler: Box<dyn HandleChangeEvent>) -> Result<NotifyChangeEvents> {
        let mut config = NotifyConfig::default();
        config.with_compare_contents(true);

        let mut modification_cache: HashMap<PathBuf, SystemTime> = HashMap::new();

        let mut watcher = RecommendedWatcher::new(move |res: NotifyResult<NotifyEvent>| {
            match res {
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
                                                handler.handle_event(change_event);
                                            } else {
                                                println!("Ignoring...");
                                            }
                                        } else {
                                            println!("Not in cache! {:?}", path);
                                            let change_event = ChangeEvent { };
                                            handler.handle_event(change_event);
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
        }, config).expect("Unable to create watcher.");

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
