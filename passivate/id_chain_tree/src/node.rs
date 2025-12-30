use crate::id_chain::IdChain;
use crate::tree::ChainLink;

#[derive(Debug, PartialEq, Eq)]
pub struct Node<TValue>
{
    pub(crate) value: TValue
}

impl<TValue> Node<TValue>
{
    pub fn new(value: TValue) -> Self
    {
        Self { value }
    }
}

impl<TLink: ChainLink, TValue> IdChain for Node<TValue>
where
    TValue: IdChain<Link = TLink>
{
    type Link = TLink;

    fn chain(&self) -> &[Self::Link]
    {
        self.value.chain()
    }
}
