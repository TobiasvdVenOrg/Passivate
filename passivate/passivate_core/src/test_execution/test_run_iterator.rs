use std::io::{BufReader, Lines};
use std::io::Error as IoError;

use duct::ReaderHandle;

pub struct TestRunIterator {
    stdout: Lines<BufReader<ReaderHandle>>
}

impl TestRunIterator {
    pub fn new(stdout: Lines<BufReader<ReaderHandle>>) -> Self {
        Self { stdout }
    }
}

impl Iterator for TestRunIterator {
    type Item = Result<String, IoError>;

    fn next(&mut self) -> Option<Self::Item> {
        self.stdout.next()
    }
}