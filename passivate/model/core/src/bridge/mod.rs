use std::fmt::Debug;

pub trait BridgeDerives = Clone + Debug + Eq + PartialEq;

pub trait HypPath
{
    type TId: BridgeDerives;

    fn path(&self) -> Self::TId;
}

pub trait Bridge
{
    type TId: BridgeDerives;
    type TOutput: HypPath<TId = Self::TId> + BridgeDerives;
    type THypInfo: HypPath<TId = Self::TId> + BridgeDerives;
}

pub trait HypSessionBridge<TBridge: Bridge>
{
    fn start_run(&mut self);
    fn output(&mut self, compilation: TBridge::TOutput);
    fn hyp(&mut self, hyp_node: TBridge::THypInfo);
    fn complete_run(&mut self);
}

pub trait HypRunBridge<TBridge: Bridge>
{
    fn run_hyps(&self);
}
