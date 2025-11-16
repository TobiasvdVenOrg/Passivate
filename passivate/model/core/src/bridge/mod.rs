pub trait Bridge
{
    type TProject;
}

pub trait HypSessionBridge<TBridge: Bridge>
{
    fn start_session(&self);
    fn project_exists(&self, project: TBridge::TProject);
    fn complete_session(&self);
}

pub trait HypRunBridge<TBridge: Bridge>
{
    fn run_hyps(&self);
}
