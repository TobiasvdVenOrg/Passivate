use super::SingleTest;

#[derive(Clone)]
#[derive(Debug)]
pub struct ActiveTestRun {
    pub tests: Vec<SingleTest>
}

impl ActiveTestRun {
    pub fn start(&mut self) {
        self.tests.clear();
    }
}