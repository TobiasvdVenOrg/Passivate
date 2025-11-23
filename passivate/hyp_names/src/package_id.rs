use std::ops::Deref;

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct PackageId(String);

impl Deref for PackageId
{
    type Target = str;

    fn deref(&self) -> &Self::Target
    {
        &self.0
    }
}

impl<T: Into<String>> From<T> for PackageId
{
    fn from(value: T) -> Self
    {
        Self(value.into())
    }
}
