use std::rc::Weak;

use passivate_model_bridge::Bridge;
use passivate_model_core::hyp::Hyp;

pub enum PassivateStateChange<TBridge: Bridge>
{
    HypSelected(Weak<Hyp<TBridge>>),
    HypDetailsChanged(Weak<Hyp<TBridge>>)
}
