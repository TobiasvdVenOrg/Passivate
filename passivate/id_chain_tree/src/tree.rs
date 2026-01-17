use std::fmt::Debug;

use crate::id_chain::{Depth, IdChain};
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

    pub fn iter(&self) -> impl Iterator<Item = &TValue>
    {
        self.values.iter().map(|node| &node.value)
    }

    pub fn iter_nodes<'a>(&'a self) -> impl Iterator<Item = NodeView<'a, TLink, TValue>>
    {
        self.values.iter().map(|node| NodeView::new(node, self))
    }

    pub fn get(&self, chain: &[TLink]) -> Option<&TValue>
    {
        self.find_node(chain).map(|node| &node.value)
    }

    pub fn get_mut(&mut self, chain: &[TLink]) -> Option<&mut TValue>
    {
        self.find_node_mut(chain).map(|node| &mut node.value)
    }

    pub fn get_node<'a>(&'a self, chain: &[TLink]) -> Option<NodeView<'a, TLink, TValue>>
    {
        self.find_node(chain).map(|node| NodeView::new(node, self))
    }

    fn find_node(&self, chain: &[TLink]) -> Option<&Node<TValue>>
    {
        self.values.iter().find(|e| e.chain() == chain)
    }

    fn find_node_mut(&mut self, chain: &[TLink]) -> Option<&mut Node<TValue>>
    {
        self.values.iter_mut().find(|e| e.chain() == chain)
    }

    pub(crate) fn children(&self, node: &Node<TValue>) -> impl Iterator<Item = &Node<TValue>>
    {
        let index = self
            .values
            .iter()
            .position(|n| n.chain() == node.chain())
            .expect("Tree::children was called with a node that isn't in the tree");

        self.values.iter().skip(index + 1).take_while(|n| n.depth() >= node.depth())
    }
}
