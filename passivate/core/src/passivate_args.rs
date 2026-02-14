use camino::Utf8PathBuf;
use clap;

#[derive(Default, clap::Parser, Debug, bon::Builder)]
pub struct PassivateArgs
{
    pub root_directory: Option<Utf8PathBuf>,
    pub config_directory: Option<Utf8PathBuf>,
    pub passivate_directory: Option<Utf8PathBuf>
}
