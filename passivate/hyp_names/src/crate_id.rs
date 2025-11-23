use std::ops::Deref;

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct CrateId(String);

impl Deref for CrateId
{
    type Target = str;

    fn deref(&self) -> &Self::Target
    {
        &self.0
    }
}

impl<T: Into<String>> From<T> for CrateId
{
    fn from(value: T) -> Self
    {
        Self(value.into())
    }
}
