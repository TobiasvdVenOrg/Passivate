use std::io::{BufRead, BufReader};
use std::io::Error as IoError;
use std::rc::Rc;

use duct::ReaderHandle;

pub struct TestRunIterator {
    stdout: Option<BufReader<ReaderHandle>>,
    buffer: Rc<String>
}

impl TestRunIterator {
    pub fn new(reader_handle: ReaderHandle) -> Self {
        // TODO: Consider rewriting without BufReader, buffering may slow down responsiveness?
        let reader = BufReader::new(reader_handle);
        Self { stdout: Some(reader), buffer: Rc::new(String::new()) }
    }

    pub fn kill(&mut self) -> Result<(), IoError> {
        let handle = self.stdout.take().unwrap().into_inner();
        handle.kill()
    }
}

impl Iterator for TestRunIterator {
    type Item = Result<Rc<String>, IoError>;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(reader) = &mut self.stdout {
            let buffer = match Rc::get_mut(&mut self.buffer) {
                Some(buffer) => { buffer.clear(); buffer },
                None => {
                    self.buffer = Rc::new(String::new());
                    Rc::make_mut(&mut self.buffer)
                }
            };

            buffer.clear();
            let result = reader.read_line(buffer);

            return match result {
                Ok(0) => None,
                Ok(_size) => Some(Ok(Rc::clone(&self.buffer))),
                Err(error) => Some(Err(error)),
            }
        }

        None
    }
}