use std::borrow::Cow;

use passivate_id_chain_tree::id_chain::IdChain;

use crate::hyp_id::HypId;

#[derive(Debug, Clone, Eq, PartialEq)]
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
            HypNameStrategy::Default | HypNameStrategy::NameOnly => Cow::Borrowed(id.chain().last().unwrap()),
            HypNameStrategy::QualifiedWithoutCrate { separator } => Cow::Owned(id.without_package_crate().join(separator)),
            HypNameStrategy::FullyQualified { separator } => Cow::Owned(id.chain().join(separator))
        }
    }
}
