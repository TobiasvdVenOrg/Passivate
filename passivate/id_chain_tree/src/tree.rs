use core::slice;
use std::fmt::Debug;

use crate::entry::Entry;
use crate::id_chain::IdChain;
use crate::node::Node;
use crate::node_view::NodeView;

pub trait ChainLink = Debug + Eq + Sized;

#[derive(Debug, PartialEq, Eq)]
pub struct Tree<TLink: ChainLink, TValue: IdChain<Link = TLink>>
{
    values: Vec<Node<TValue>>
}

impl<TLink: ChainLink, TValue: IdChain<Link = TLink>> Tree<TLink, TValue>
{
    pub fn new() -> Self
    {
        Self { values: Vec::new() }
    }

    pub fn insert(&mut self, element: TValue)
    {
        self.values.push(Node::new(element));
    }

    pub fn iter(&self) -> Iter<'_, TLink, TValue>
    {
        let inner = self.values[..].iter();

        Iter { inner }
    }

    pub fn iter_nodes(&self) -> Iter<'_, TLink, TValue>
    {
        todo!();
    }

    pub fn entry<'a>(&'a self, chain: &'a [TLink]) -> Entry<'a, TLink, TValue>
    {
        let node = self.values.iter().find(|e| e.chain() == chain);
        Entry::new(node, self, chain)
    }
}

#[derive(Debug)]
pub struct Iter<'a, TLink, TValue: 'a>
where
    TLink: ChainLink,
    TValue: 'a + IdChain<Link = TLink>
{
    inner: slice::Iter<'a, Node<TValue>>
}

#[derive(Debug)]
pub struct NodeIter<'a, TLink, TValue>
where
    TLink: ChainLink,
    TValue: 'a + IdChain<Link = TLink>
{
    inner: slice::Iter<'a, Node<TValue>>,
    tree: &'a Tree<TLink, TValue>
}

impl<'a, TLink, TValue: 'a> Iter<'a, TLink, TValue>
where
    TLink: ChainLink,
    TValue: 'a + IdChain<Link = TLink>
{
    pub fn empty() -> Self
    {
        let inner = [].iter();
        Self { inner }
    }
}

impl<'a, TLink: ChainLink, TValue> Iterator for Iter<'a, TLink, TValue>
where
    TValue: IdChain<Link = TLink>
{
    type Item = &'a TValue;

    fn next(&mut self) -> Option<Self::Item>
    {
        self.inner.next().map(|node| &node.value)
    }
}

impl<'a, TLink: ChainLink, TValue> Iterator for NodeIter<'a, TLink, TValue>
where
    TValue: IdChain<Link = TLink>
{
    type Item = NodeView<'a, TLink, TValue>;

    fn next(&mut self) -> Option<Self::Item>
    {
        self.inner.next().map(|node| NodeView::new(node, self.tree))
    }
}
