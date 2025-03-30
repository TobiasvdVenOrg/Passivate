
#[derive(Clone, Debug, PartialEq)]
pub struct TestId {
    name: String
}

impl TestId {
    pub fn new(name: String) -> Self {
        Self { name }
    }
}
