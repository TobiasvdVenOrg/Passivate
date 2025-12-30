use passivate_model_bridge::bridge::Bridge;

use crate::hyp::Hyp;

#[derive(Debug)]
pub enum HypSessionChange<'a, TBridge: Bridge>
{
    NewHyp(&'a Hyp<TBridge>),
    HypUpdated(&'a Hyp<TBridge>)
}
