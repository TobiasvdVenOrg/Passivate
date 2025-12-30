use std::borrow::Cow;
use std::ops::Deref;

use passivate_id_chain_tree::id_chain::IdChain;

use crate::crate_id::CrateId;
use crate::hyp_name_strategy::HypNameStrategy;
use crate::package_id::PackageId;

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct HypId
{
    parts: Vec<String>
}

impl HypId
{
    pub fn new(package_id: impl Into<PackageId>, crate_id: impl Into<CrateId>, hyp_name: impl Into<String>) -> Self
    {
        let package_id = package_id.into().to_string();
        let crate_id = crate_id.into().to_string();
        let mut name_parts: Vec<String> = hyp_name.into().split_terminator("::").map(String::from).collect();

        let mut parts = vec![package_id, crate_id];
        parts.append(&mut name_parts);

        Self { parts }
    }

    pub fn package_id(&self) -> PackageId
    {
        PackageId::from(&self.parts[0])
    }

    pub fn crate_id(&self) -> CrateId
    {
        CrateId::from(&self.parts[1])
    }

    pub fn package_crate_name(&self) -> &[String]
    {
        &self.parts[.. 2]
    }

    pub fn without_package_crate(&self) -> &[String]
    {
        &self.parts[2 ..]
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

impl IdChain for HypId
{
    type Link = String;

    fn chain(&self) -> &[Self::Link]
    {
        &self.parts
    }
}

impl Deref for HypId
{
    type Target = [String];

    fn deref(&self) -> &Self::Target
    {
        self.chain()
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
