use passivate_hyp_names::hyp_id::HypId;

#[derive(Clone, PartialEq, Debug)]
pub enum HypRunTrigger
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
