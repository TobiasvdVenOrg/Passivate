use crate::bridge::Bridge;

pub struct HypRunRequest<TBridge: Bridge>
{
    pub kind: HypRunRequestKind<TBridge>,
    pub options: HypRunOptions
}

impl<TBridge: Bridge> HypRunRequest<TBridge>
{
    pub fn all(options: HypRunOptions) -> Self
    {
        Self {
            kind: HypRunRequestKind::All,
            options
        }
    }

    pub fn single(hyp_id: TBridge::Id, options: HypRunOptions) -> Self
    {
        Self {
            kind: HypRunRequestKind::Single { hyp_id },
            options
        }
    }
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
