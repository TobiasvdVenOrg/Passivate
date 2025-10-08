use passivate_hyp_names::hyp_id::HypId;

use super::SingleTest;

#[derive(Debug, Clone)]
pub enum TestRunEvent
{
    Start,
    StartSingle
    {
        hyp: HypId,
        clear_tests: bool
    },
    Compiling(String),
    BuildError(String),
    TestFinished(SingleTest),
    TestsCompleted,
    NoTests,
    ErrorOutput
    {
        hyp: HypId,
        message: String
    }
}
