use crate::bridge::Bridge;
use crate::hyp::Hyp;
use crate::hyp_session::Project;

#[derive(Debug)]
pub enum HypSessionChange<'a, TBridge: Bridge>
{
    NewProject(&'a Project<TBridge>),
    HypUpdated(&'a Hyp)
}
