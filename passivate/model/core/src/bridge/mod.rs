use std::fmt::Debug;

pub trait BridgeDerives = Clone + Debug + Eq + PartialEq;

pub trait ProjectId
{
    type TId;

    fn id(&self) -> &Self::TId;
}

pub trait Bridge
{
    type TProjectId: BridgeDerives;
    type TProjectInfo: ProjectId<TId = Self::TProjectId> + BridgeDerives;
    type TWorkspaceCompilation: BridgeDerives;
    type TProjectCompilation: ProjectId<TId = Self::TProjectId> + BridgeDerives;
    type THypNode: ProjectId<TId = Self::TProjectId> + BridgeDerives;
}

pub trait HypSessionBridge<TBridge: Bridge>
{
    fn start_run(&mut self);
    fn project_exists(&mut self, project: TBridge::TProjectInfo);
    fn workspace_compilation(&mut self, compilation: TBridge::TWorkspaceCompilation);
    fn project_compilation(&mut self, compilation: TBridge::TProjectCompilation);
    fn hyp_node_exists(&mut self, hyp_node: TBridge::THypNode);
    fn complete_run(&mut self);
}

pub trait HypRunBridge<TBridge: Bridge>
{
    fn run_hyps(&self);
}
