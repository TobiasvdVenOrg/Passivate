use std::fmt::Display;

use crate::id_chain::IdChain;
use crate::tree::{ChainLink, Tree};

impl<TLink: ChainLink, TValue: IdChain<Link = TLink>> Display for Tree<TLink, TValue>
{
    fn fmt(&self, _f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
    {
        todo!()
    }
}
