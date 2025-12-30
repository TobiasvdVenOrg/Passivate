use crate::bridge::Bridge;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct HypReport<TBridge: Bridge>
{
    id: TBridge::Id,
    hyp_info: TBridge::HypInfo
}
