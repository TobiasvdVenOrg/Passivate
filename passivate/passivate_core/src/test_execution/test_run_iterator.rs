use std::io::{BufRead, BufReader};
use std::io::Error as IoError;

use duct::ReaderHandle;

pub struct TestRunIterator {
    stdout: Option<BufReader<ReaderHandle>>,
    buffer: String
}

impl TestRunIterator {
    pub fn new(reader_handle: ReaderHandle) -> Self {
        // TODO: Consider rewriting without BufReader, buffering may slow down responsiveness?
        let reader = BufReader::new(reader_handle);
        Self { stdout: Some(reader), buffer: String::new() }
    }

    pub fn kill(&mut self) -> Result<(), IoError> {
        let handle = self.stdout.take().unwrap().into_inner();
        handle.kill()
    }
}

impl Iterator for TestRunIterator {
    type Item = Result<String, IoError>;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(reader) = &mut self.stdout {
            self.buffer.clear();
            let result = reader.read_line(&mut self.buffer);

            return match result {
                Ok(0) => None,
                Ok(_size) => Some(Ok(self.buffer.clone())),
                Err(error) => Some(Err(error)),
            }
        }

        None
    }
}