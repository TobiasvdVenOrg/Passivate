use std::rc::Weak;

use crate::bridge::Bridge;
use crate::hyp::Hyp;

#[derive(Debug)]
pub enum HypSessionChange<TBridge: Bridge>
{
    NewNode(Weak<Hyp<TBridge>>),
    HypUpdated(Weak<Hyp<TBridge>>)
}
