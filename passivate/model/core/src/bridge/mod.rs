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
    type TProjectInfo: ProjectId<T = Self::TProjectId> + BridgeDerives;
    type TWorkspaceCompilation: BridgeDerives;
    type TProjectCompilation: ProjectId<T = Self::TProjectId> + BridgeDerives;
}

pub trait HypSessionBridge<TBridge: Bridge>
{
    fn start_run(&mut self);
    fn project_exists(&mut self, project: TBridge::TProjectInfo);
    fn workspace_compilation(&mut self, compilation: TBridge::TWorkspaceCompilation);
    fn project_compilation(&mut self, compilation: TBridge::TProjectCompilation);
    fn complete_run(&mut self);
}

pub trait HypRunBridge<TBridge: Bridge>
{
    fn run_hyps(&self);
}
