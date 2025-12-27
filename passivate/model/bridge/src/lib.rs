#![feature(trait_alias)]

use std::fmt::{Debug, Display};

use passivate_id_chain_tree::id_chain::IdChain;

pub trait BridgeType = Clone + Debug + Eq + PartialEq;

pub trait Bridge
{
    type IdLink: BridgeType;
    type Id: IdChain<Link = Self::IdLink> + BridgeType;
    type Output: Display + BridgeType;
    type HypInfo: IdChain<Link = Self::IdLink> + BridgeType;
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct HypReport<TBridge: Bridge>
{
    id: TBridge::Id,
    hyp_info: TBridge::HypInfo
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct OutputReport<TBridge: Bridge>
{
    id: TBridge::Id,
    output: TBridge::Output
}

impl<TBridge: Bridge> OutputReport<TBridge>
{
    pub fn new(id: TBridge::Id, output: TBridge::Output) -> OutputReport<TBridge>
    {
        Self { id, output }
    }
}

/// Interface from a test runner implementation to communicate changes to the session state.
pub trait HypSessionBridge<TBridge: Bridge>
{
    fn start_run(&mut self);
    fn output(&mut self, output: OutputReport<TBridge>);
    fn hyp(&mut self, hyp_info: TBridge::HypInfo);
    fn complete_run(&mut self);
}

/// Interface from a session state to start test runs.
pub trait HypRunBridge<TBridge: Bridge>
{
    fn run_hyps(&self);
}
