use super::SingleTest;

#[derive(Clone)]
#[derive(Debug)]
pub struct ActiveTestRun {
    pub tests: Vec<SingleTest>
}
