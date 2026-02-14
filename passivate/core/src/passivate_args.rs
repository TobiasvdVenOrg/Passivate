use camino::Utf8PathBuf;
use clap;

#[derive(Default, clap::Parser, Debug, bon::Builder)]
pub struct PassivateArgs
{
    pub root_directory: Option<Utf8PathBuf>,

    pub target_directory: Option<Utf8PathBuf>
}
