use crate::id_chain::IdChain;
use crate::node::Node;
use crate::tree::{ChainLink, Iter, Tree};

#[derive(Debug)]
pub struct NodeView<'a, TLink, TValue>
where
    TLink: ChainLink,
    TValue: IdChain<Link = TLink>
{
    node: &'a Node<TValue>,
    tree: &'a Tree<TLink, TValue>
}

impl<'a, TLink, TValue> NodeView<'a, TLink, TValue>
where
    TLink: ChainLink,
    TValue: IdChain<Link = TLink>
{
    pub(crate) fn new(node: &'a Node<TValue>, tree: &'a Tree<TLink, TValue>) -> Self
    {
        Self { node, tree }
    }

    pub fn iter_children(&self) -> Iter<'_, TLink, TValue>
    {
        todo!()
    }
}
