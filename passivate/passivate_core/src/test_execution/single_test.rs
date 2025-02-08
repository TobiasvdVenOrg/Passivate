use std::path::{Path, PathBuf};
use crate::test_execution::SingleTestStatus;

#[derive(Clone)]
#[derive(Debug)]
pub struct SingleTest {
    pub name: String,
    pub status: SingleTestStatus,
    pub file: PathBuf,
    pub line: u32
}

impl SingleTest {
    pub fn new(name: String, status: SingleTestStatus, file: &Path, line: u32) -> SingleTest {
        SingleTest {
            name,
            status,
            file: file.to_path_buf(),
            line
        }
    }
}