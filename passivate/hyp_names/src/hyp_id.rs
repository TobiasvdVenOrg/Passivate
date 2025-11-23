use std::borrow::Cow;

use crate::crate_id::CrateId;
use crate::package_id::PackageId;

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

impl AsRef<HypNameStrategy> for HypNameStrategy
{
    fn as_ref(&self) -> &HypNameStrategy
    {
        self
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
            HypNameStrategy::FullyQualified { separator } =>
            {
                Cow::Owned(format!(
                    "{:?}{}{:?}{}{}",
                    id.package_id(),
                    separator,
                    id.crate_id(),
                    separator,
                    id.parts.join(separator)
                ))
            }
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct HypId
{
    package_id: PackageId,
    crate_id: CrateId,
    parts: Vec<String>
}

impl HypId
{
    pub fn new(package_id: impl Into<PackageId>, crate_id: impl Into<CrateId>, hyp_name: impl Into<String>) -> Self
    {
        let parts: Vec<String> = hyp_name.into().split_terminator("::").map(String::from).collect();

        Self {
            package_id: package_id.into(),
            crate_id: crate_id.into(),
            parts
        }
    }

    pub fn package_id(&self) -> &PackageId
    {
        &self.package_id
    }

    pub fn crate_id(&self) -> &CrateId
    {
        &self.crate_id
    }

    pub fn package_crate_name(&self, separator: impl AsRef<str>) -> String
    {
        format!("{:?}{}{:?}", self.package_id(), separator.as_ref(), self.crate_id())
    }

    pub fn name<'a>(&'a self, strategy: impl AsRef<HypNameStrategy>) -> Cow<'a, str>
    {
        strategy.as_ref().convert(self)
    }

    pub fn fully_qualified(&self, separator: impl AsRef<str>) -> String
    {
        let strategy = HypNameStrategy::FullyQualified {
            separator: separator.as_ref().to_string()
        };

        strategy.convert(self).to_string()
    }
}

#[cfg(test)]
mod tests
{
    use crate::hyp_id::HypNameStrategy;
    use crate::{test_id, test_name};

    #[test]
    pub fn example_unit_test_id()
    {
        let id = test_id!().fully_qualified("::");

        assert_eq!(
            "passivate_hyp_names::passivate_hyp_names::hyp_id::tests::example_unit_test_id",
            id
        );
    }

    #[test]
    pub fn id_as_name_only_from_unit_test()
    {
        let id = test_id!();

        let name = id.name(&HypNameStrategy::NameOnly);

        assert_eq!("id_as_name_only_from_unit_test", name);
    }

    #[test]
    pub fn id_as_fully_qualified_from_unit_test()
    {
        let id = test_id!();
        let strategy = HypNameStrategy::FullyQualified {
            separator: "+".to_string()
        };

        let name = id.name(&strategy);

        assert_eq!(
            "passivate_hyp_names+passivate_hyp_names+hyp_id+tests+id_as_fully_qualified_from_unit_test",
            name
        );
    }

    #[test]
    pub fn id_as_qualified_without_crate_from_unit_test()
    {
        let id = test_id!();
        let strategy = HypNameStrategy::QualifiedWithoutCrate {
            separator: "+".to_string()
        };

        let name = id.name(&strategy);

        assert_eq!("hyp_id+tests+id_as_qualified_without_crate_from_unit_test", name);
    }

    #[test]
    pub fn example_unit_test_name()
    {
        let name = test_name!();

        assert_eq!("example_unit_test_name", name);
    }
}
