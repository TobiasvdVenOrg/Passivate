use crate::id_chain::IdChain;
use crate::node::Node;
use crate::node_view::NodeView;
use crate::tree::{ChainLink, Tree};

pub struct Entry<'a, TLink: ChainLink, TValue>
where
    TValue: IdChain<Link = TLink>
{
    node: Option<&'a Node<TValue>>,
    tree: &'a Tree<TLink, TValue>,
    _chain: &'a [TLink]
}

impl<'a, TLink: ChainLink, TValue> Entry<'a, TLink, TValue>
where
    TValue: IdChain<Link = TLink>
{
    pub(crate) fn new(node: Option<&'a Node<TValue>>, tree: &'a Tree<TLink, TValue>, chain: &'a [TLink]) -> Self
    {
        Self { node, tree, _chain: chain }
    }

    pub fn or_none(&self) -> Option<&'a TValue>
    {
        self.node.map(|node| &node.value)
    }

    pub fn node_or_none(&self) -> Option<NodeView<'a, TLink, TValue>>
    {
        self.node.map(|node| NodeView::new(node, &self.tree))
    }

    pub fn unwrap(&self) -> &'a TValue
    {
        self.or_none().unwrap()
    }
}
