use crate::bridge::Bridge;

pub struct HypRunRequest<TBridge: Bridge>
{
    pub kind: HypRunRequestKind<TBridge>,
    pub options: HypRunOptions
}

#[derive(Default)]
pub struct HypRunOptions
{
    pub update_snapshots: bool,
    pub compute_coverage: bool
}

#[derive(Clone, PartialEq, Debug)]
pub enum HypRunRequestKind<TBridge: Bridge>
{
    All,
    Single
    {
        hyp_id: TBridge::Id
    }
}
