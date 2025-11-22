use std::fmt::Debug;

pub trait BridgeDerives = Clone + Debug + Eq + PartialEq;

pub trait ProjectId
{
    type T;

    fn id(&self) -> &Self::T;
}

pub trait Bridge
{
    type TProjectId: BridgeDerives;
    type TProject: ProjectId<T = Self::TProjectId> + BridgeDerives;
    type TWorkspaceCompilation: BridgeDerives;
    type TProjectCompilation: ProjectId<T = Self::TProjectId> + BridgeDerives;
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
