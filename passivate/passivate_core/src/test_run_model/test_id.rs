#[derive(Clone, Debug, PartialEq)]
pub struct TestId
{
    name: String
}

impl TestId
{
    pub fn new<T: ToString>(name: T) -> Self
    {
        Self { name: name.to_string() }
    }

    pub fn get_name(&self) -> String
    {
        self.name.clone()
    }
}
