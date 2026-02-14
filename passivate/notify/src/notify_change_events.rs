use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;
use std::time::SystemTime;

use camino::{Utf8Path, Utf8PathBuf};
use notify::{
    Config as NotifyConfig,
    Event as NotifyEvent,
    RecommendedWatcher,
    RecursiveMode,
    Result as NotifyResult,
    Watcher
};
use passivate_model_bridge::source_change_bridge::FileChangedBridge;

use crate::notify_change_events_errors::NotifyChangeEventsError;

pub struct NotifyChangeEvents
{
    watcher: RecommendedWatcher,
    path: Utf8PathBuf
}

impl NotifyChangeEvents
{
    pub fn start_watching(
        path: Utf8PathBuf,
        bridge: impl FileChangedBridge + Send + Sync + 'static
    ) -> Result<NotifyChangeEvents, NotifyChangeEventsError>
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
                                if let Some(last_modification) = modification_cache.get(path)
                                {
                                    if &modified > last_modification
                                    {
                                        bridge.file_changed(path.clone());
                                    }
                                }
                                else
                                {
                                    bridge.file_changed(path.clone());
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
            Ok(watcher) =>
            {
                let watcher = Self::start_watcher(watcher, &path)?;

                Ok(NotifyChangeEvents { watcher, path })
            }
            Err(notify_error) => Err(NotifyChangeEventsError::invalid_path(path, notify_error))
        }
    }

    pub fn stop(&mut self) -> Result<(), NotifyChangeEventsError>
    {
        match self.watcher.unwatch(self.path.as_std_path())
        {
            Ok(_) => Ok(()),
            Err(notify_error) => Err(NotifyChangeEventsError::invalid_path(self.path.clone(), notify_error))
        }
    }

    fn start_watcher<T: Watcher>(mut watcher: T, path: &Utf8Path) -> Result<T, NotifyChangeEventsError>
    {
        let watch_result = watcher.watch(path.as_std_path(), RecursiveMode::Recursive);

        match watch_result
        {
            Ok(_) => Ok(watcher),
            Err(notify_error) => Err(NotifyChangeEventsError::invalid_path(path.to_path_buf(), notify_error))
        }
    }
}
