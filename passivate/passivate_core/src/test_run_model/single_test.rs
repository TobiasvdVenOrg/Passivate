use super::{SingleTestStatus, TestId};

#[derive(Clone)]
#[derive(Debug)]
#[derive(PartialEq)]
pub struct SingleTest {
    id: TestId,
    pub name: String,
    pub status: SingleTestStatus
}

impl SingleTest {
    pub fn new(name: String, status: SingleTestStatus) -> Self {
        Self { id: TestId::new(name.clone()), name, status }
    }

    pub fn id(&self) -> TestId {
        self.id.clone()
    }
}
