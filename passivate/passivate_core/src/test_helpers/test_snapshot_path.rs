use camino::Utf8PathBuf;

pub struct TestSnapshotPath
{
    pub path: Utf8PathBuf,
    pub kind: TestSnapshotPathKind
}

pub enum TestSnapshotPathKind
{
    Normal,
    RelativeToOutput,
    RelativeToWorkspace
}

impl Default for TestSnapshotPath
{
    fn default() -> Self
    {
        Self {
            path: Utf8PathBuf::new().join("tests").join("snapshots"),
            kind: TestSnapshotPathKind::RelativeToWorkspace
        }
    }
}
impl TestSnapshotPath
{
    pub fn new<T: AsRef<str>>(path: T, kind: TestSnapshotPathKind) -> Self
    {
        Self {
            path: Utf8PathBuf::from(path.as_ref()),
            kind
        }
    }
    pub fn normal<T: AsRef<str>>(path: T) -> Self
    {
        Self::new(path, TestSnapshotPathKind::Normal)
    }

    pub fn relative_to_output<T: AsRef<str>>(path: T) -> Self
    {
        Self::new(path, TestSnapshotPathKind::RelativeToOutput)
    }

    pub fn relative_to_workspace<T: AsRef<str>>(path: T) -> Self
    {
        Self::new(path, TestSnapshotPathKind::RelativeToWorkspace)
    }
}
