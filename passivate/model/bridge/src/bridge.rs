use std::fmt::Display;

use passivate_id_chain_tree::id_chain::IdChain;

use crate::BridgeType;
use crate::bridge_hyp::BridgeHyp;
use crate::hyp_run_bridge::HypRunBridge;

pub trait Bridge
{
    type IdLink: BridgeType;
    type Id: IdChain<Link = Self::IdLink> + Display + BridgeType;
    type Output: Display + BridgeType;
    type HypInfo: BridgeHyp<Id = Self::Id> + IdChain<Link = Self::IdLink> + BridgeType;
    type HypRunner: HypRunBridge;
}
