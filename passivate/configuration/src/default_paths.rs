use camino::Utf8PathBuf;

#[derive(Debug, Clone)]
pub struct DefaultPaths
{
    pub root: Utf8PathBuf,
    pub passivate: Utf8PathBuf,
    pub target: Utf8PathBuf
}

impl DefaultPaths
{
    pub fn new(root: Utf8PathBuf) -> Self
    {
        let passivate = root.join(".passivate");
        let target = passivate.join("target");

        Self { root, passivate, target }
    }
}

#[cfg(feature = "testing")]
pub fn stub() -> DefaultPaths
{
    DefaultPaths::new(Utf8PathBuf::from("testing_stub"))
}
