use passivate_hyp_names::hyp_id::HypId;

use crate::{single_hyp::SingleHyp, test_run::FailedTestRun};


#[derive(Debug, Clone)]
pub enum HypRunEvent
{
    Start,
    StartSingle
    {
        hyp: HypId,
        clear_tests: bool
    },
    Compiling(String),
    BuildError(String),
    TestFinished(SingleHyp),
    TestsCompleted,
    NoTests,
    ErrorOutput
    {
        hyp: HypId,
        message: String
    },
    HypRunError(FailedTestRun)
}
