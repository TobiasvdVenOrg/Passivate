use std::path::PathBuf;

use passivate_delegation::Tx;

use crate::source_change_event::SourceChangeEvent;

pub trait FileChangedBridge
{
    fn file_changed(&self, file: PathBuf);
}

impl<TTx> FileChangedBridge for TTx
where
    TTx: Tx<SourceChangeEvent>
{
    fn file_changed(&self, file: PathBuf)
    {
        log::info!("File change: {file:?}");

        self.send(SourceChangeEvent::File(file));
    }
}
