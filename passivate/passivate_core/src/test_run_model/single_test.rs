use std::path::PathBuf;

use super::SingleTestStatus;

#[derive(Clone)]
#[derive(Debug)]
#[derive(PartialEq)]
pub struct SingleTest {
    pub name: String,
    pub status: SingleTestStatus,
    pub snapshot: Option<PathBuf>
}

impl SingleTest {
    pub fn new(name: String, status: SingleTestStatus) -> Self {
        Self { name, status, snapshot: None }
    }

    pub fn with_snapshot(name: String, status: SingleTestStatus, snapshot: PathBuf) -> Self {
        Self { name, status, snapshot: Some(snapshot) }
    }
}
