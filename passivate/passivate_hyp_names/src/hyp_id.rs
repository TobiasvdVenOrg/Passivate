use std::borrow::Cow;

use thiserror::Error;

pub enum HypNameStrategy
{
    Default,
    NameOnly,
    QualifiedWithoutCrate
    {
        separator: String
    },
    FullyQualified 
    {
        separator: String
    }
}

impl HypNameStrategy
{
    pub fn convert<'a>(&self, id: &'a HypId) -> Cow<'a, str>
    {
        match self
        {
            HypNameStrategy::Default | HypNameStrategy::NameOnly => Cow::Borrowed(id.parts.last().unwrap()),
            HypNameStrategy::QualifiedWithoutCrate { separator } => Cow::Owned(id.parts.join(separator)),
            HypNameStrategy::FullyQualified { separator } => Cow::Owned(format!("{}{}{}", id.get_crate_name(separator), separator, id.parts.join(separator)))
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct HypId
{
    crate_parts: Vec<String>,
    parts: Vec<String>
}

impl HypId
{
    pub fn new<TCrateName, TValue>(crate_name: TCrateName, value: TValue) -> Result<Self, HypIdError>
        where 
            TCrateName: AsRef<str>,
            TValue: AsRef<str>
    {
        let crate_parts = crate_name.as_ref().split_terminator("::").map(String::from).collect();
        let parts: Vec<String> = value.as_ref().split_terminator("::").map(String::from).collect();

        Ok(Self { crate_parts, parts })
    }

    pub fn get_crate_name<TSeparator: AsRef<str>>(&self, separator: TSeparator) -> String
    {
        self.crate_parts.join(separator.as_ref()).to_string()
    }

    pub fn get_name<'a>(&'a self, name_only: &'a HypNameStrategy) -> Cow<'a, str>
    {
        name_only.convert(self)
    }

    pub fn get_fully_qualified<TSeparator: AsRef<str>>(&self, separator: TSeparator) -> String
    {
        let strategy = HypNameStrategy::FullyQualified { separator: separator.as_ref().to_string() };

        strategy.convert(self).to_string()
    }
}

#[derive(Error, Debug, PartialEq)]
pub enum HypIdError
{
    #[error("hyp id was not in a valid format: {0}")]
    InvalidFormat(String)
}

#[cfg(test)]
mod tests
{
    use crate::hyp_id::{HypId, HypIdError, HypNameStrategy};
    use crate::{test_id, test_name};
    use rstest::*;

    #[test]
    pub fn example_unit_test_id()
    {
        let id = test_id!().get_fully_qualified("::");

        assert_eq!("passivate_hyp_names::hyp_id::tests::example_unit_test_id", id);
    }

    #[test]
    pub fn id_as_name_only_from_unit_test()
    {
        let id = test_id!();

        let name = id.get_name(&HypNameStrategy::NameOnly);

        assert_eq!("id_as_name_only_from_unit_test", name);
    }

    #[test]
    pub fn id_as_fully_qualified_from_unit_test()
    {
        let id = test_id!();
        let strategy = HypNameStrategy::FullyQualified { separator: "+".to_string() };

        let name = id.get_name(&strategy);

        assert_eq!("passivate_hyp_names+hyp_id+tests+id_as_fully_qualified_from_unit_test", name);
    }

    #[test]
    pub fn id_as_qualified_without_crate_from_unit_test()
    {
        let id = test_id!();
        let strategy = HypNameStrategy::QualifiedWithoutCrate { separator: "+".to_string() };

        let name = id.get_name(&strategy);

        assert_eq!("hyp_id+tests+id_as_qualified_without_crate_from_unit_test", name);
    }

    #[test]
    pub fn example_unit_test_name()
    {
        let name = test_name!();

        assert_eq!("example_unit_test_name", name);
    }
}
