use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::time::SystemTime;

use notify::{Config as NotifyConfig, Event as NotifyEvent, RecommendedWatcher, RecursiveMode, Result as NotifyResult, Watcher};
use passivate_core::change_events::ChangeEvent;
use passivate_delegation::Tx;

use crate::passivate_notify::NotifyChangeEventsError;

pub struct NotifyChangeEvents
{
    watcher: RecommendedWatcher,
    path: PathBuf
}

impl NotifyChangeEvents
{
    pub fn new(path: &Path, mut change_events: Tx<ChangeEvent>) -> Result<NotifyChangeEvents, NotifyChangeEventsError>
    {
        let mut modification_cache: HashMap<PathBuf, SystemTime> = HashMap::new();

        let config = NotifyConfig::default().with_compare_contents(true);

        let watcher = RecommendedWatcher::new(
            move |event: NotifyResult<NotifyEvent>| {
                match event
                {
                    Ok(event) =>
                    {
                        for path in &event.paths
                        {
                            let extension = path.extension();

                            if let Some(extension) = extension
                                && extension == "rs"
                                && let Ok(metadata) = fs::metadata(path)
                                && let Ok(modified) = metadata.modified()
                            {
                                if let Some(last_modification) = modification_cache.get(path.as_path())
                                {
                                    if &modified > last_modification
                                    {
                                        let change_event = ChangeEvent::DefaultRun;

                                        change_events.send(change_event);
                                    }
                                }
                                else
                                {
                                    let change_event = ChangeEvent::DefaultRun;

                                    change_events.send(change_event);
                                }

                                modification_cache.insert(path.clone(), modified);
                            }
                        }
                    }
                    Err(error) =>
                    {
                        println!("{:?}", error);
                    }
                }
            },
            config
        );

        match watcher
        {
            Ok(mut watcher) =>
            {
                watcher = Self::start_watcher(watcher, path)?;

                Ok(NotifyChangeEvents {
                    watcher,
                    path: path.to_path_buf()
                })
            }
            Err(notify_error) => Err(NotifyChangeEventsError::invalid_path(path, notify_error))
        }
    }

    pub fn stop(&mut self) -> Result<(), NotifyChangeEventsError>
    {
        match self.watcher.unwatch(self.path.as_path())
        {
            Ok(_) => Ok(()),
            Err(notify_error) => Err(NotifyChangeEventsError::invalid_path(self.path.as_path(), notify_error))
        }
    }

    fn start_watcher<T: Watcher>(mut watcher: T, path: &Path) -> Result<T, NotifyChangeEventsError>
    {
        let watch_result = watcher.watch(path, RecursiveMode::Recursive);

        match watch_result
        {
            Ok(_) => Ok(watcher),
            Err(notify_error) => Err(NotifyChangeEventsError::invalid_path(path, notify_error))
        }
    }
}
