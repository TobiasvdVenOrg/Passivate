use std::fmt::{Debug, Display};

use passivate_id_chain_tree::id_chain::IdChain;

use crate::BridgeType;
use crate::bridge_hyp::BridgeHyp;

pub trait Bridge: Debug + Send + Sync + 'static
{
    type IdLink: BridgeType;
    type Id: IdChain<Link = Self::IdLink> + Display + BridgeType;
    type Output: Display + BridgeType;
    type HypInfo: BridgeHyp<Id = Self::Id> + IdChain<Link = Self::IdLink> + BridgeType;
    type RunError: BridgeType;
}
