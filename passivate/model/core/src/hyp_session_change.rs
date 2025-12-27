use std::rc::Weak;

use passivate_model_bridge::Bridge;

use crate::hyp::Hyp;

#[derive(Debug)]
pub enum HypSessionChange<TBridge: Bridge>
{
    NewHyp(Weak<Hyp<TBridge>>),
    HypUpdated(Weak<Hyp<TBridge>>)
}
