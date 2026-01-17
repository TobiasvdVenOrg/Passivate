use crate::id_chain::{Depth, IdChain};
use crate::node::Node;
use crate::tree::{ChainLink, Tree};

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

    pub fn depth(&self) -> usize
    {
        self.node.depth()
    }

    pub fn value(&self) -> &TValue
    {
        &self.node.value
    }

    pub fn iter_children(&self) -> impl Iterator<Item = &TValue>
    {
        self.tree.children(self.node).map(|node| &node.value)
    }
}
