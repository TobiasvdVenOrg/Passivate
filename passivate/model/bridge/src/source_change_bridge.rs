use std::path::PathBuf;

use passivate_delegation::tx_rx::Tx;

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
        self.send(SourceChangeEvent::File(file));
    }
}
