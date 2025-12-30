/// Interface from a session state to start test runs.
#[mockall::automock]
pub trait HypRunBridge
{
    fn run_hyps(&self);
}
