use camino::Utf8PathBuf;
use clap::Parser;

#[derive(Parser, Debug, bon::Builder)]
#[command()]
pub struct PassivateArgs
{
    pub manifest_directory: Option<Utf8PathBuf>
}
