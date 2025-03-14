use std::io::{BufReader, Lines};
use std::io::Error as IoError;

pub struct TestRunIterator {
    stdout: Lines<BufReader<os_pipe::PipeReader>>
}

impl TestRunIterator {
    pub fn new(stdout: Lines<BufReader<os_pipe::PipeReader>>) -> Self {
        Self { stdout }
    }
}

impl Iterator for TestRunIterator {
    type Item = Result<String, IoError>;

    fn next(&mut self) -> Option<Self::Item> {
            self.stdout.next()
    }
}