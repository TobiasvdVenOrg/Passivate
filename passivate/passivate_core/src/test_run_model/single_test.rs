use super::{SingleTestStatus, TestId};

#[derive(Clone, Debug, PartialEq)]
pub struct SingleTest
{
    id: TestId,
    pub name: String,
    pub status: SingleTestStatus,
    pub output: Vec<String>
}

impl SingleTest
{
    pub fn new(name: String, status: SingleTestStatus, output: Vec<String>) -> Self
    {
        Self {
            id: TestId::new(name.clone()),
            name,
            status,
            output
        }
    }

    pub fn id(&self) -> TestId
    {
        self.id.clone()
    }
}
