use std::path::PathBuf;

pub enum SourceChangeEvent
{
    File(PathBuf)
}
