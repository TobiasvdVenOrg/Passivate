use std::fmt::Debug;

pub trait Bridge
{
    type TProject: Clone + Debug + Eq + PartialEq;
}

pub trait HypSessionBridge<TBridge: Bridge>
{
    fn start_run(&self);
    fn project_exists(&self, project: TBridge::TProject);
    fn complete_run(&self);
}

pub trait HypRunBridge<TBridge: Bridge>
{
    fn run_hyps(&self);
}
