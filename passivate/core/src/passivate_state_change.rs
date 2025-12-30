use passivate_model_bridge::bridge::Bridge;
use passivate_model_core::hyp::Hyp;

pub enum PassivateStateChange<'a, TBridge: Bridge>
{
    HypSelected(&'a Hyp<TBridge>),
    HypDetailsChanged(&'a Hyp<TBridge>)
}
