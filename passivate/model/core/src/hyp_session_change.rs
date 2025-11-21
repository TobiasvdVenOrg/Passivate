use crate::bridge::Bridge;
use crate::hyp::Hyp;

#[derive(Debug)]
pub enum HypSessionChange<'a, TBridge: Bridge>
{
    NewProject(&'a TBridge::TProject),
    HypUpdated(&'a Hyp)
}
