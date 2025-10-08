use enum_as_inner::EnumAsInner;
use passivate_hyp_names::hyp_id::HypId;

#[derive(Clone, EnumAsInner, PartialEq, Debug)]
pub enum ChangeEvent
{
    DefaultRun,
    SingleHyp
    {
        id: HypId,
        update_snapshots: bool
    },
    PinHyp
    {
        id: HypId
    },
    ClearPinnedHyps
}
