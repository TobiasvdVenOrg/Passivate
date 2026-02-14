use camino::{Utf8Path, Utf8PathBuf};

#[derive(Debug, Clone)]
pub struct DefaultPaths
{
    pub root: Utf8PathBuf,
    pub passivate: Utf8PathBuf
}

impl DefaultPaths
{
    pub fn from_root(root: Utf8PathBuf) -> Self
    {
        let passivate = root.join("..").join(".passivate");

        Self { root, passivate }
    }
}

#[cfg(feature = "testing")]
pub fn stub() -> DefaultPaths
{
    DefaultPaths {
        root: Utf8PathBuf::from("testing_stub"),
        passivate: Utf8PathBuf::from("testing_stub").join(".passivate")
    }
}
