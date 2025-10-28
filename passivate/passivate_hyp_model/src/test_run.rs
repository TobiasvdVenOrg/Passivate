use crate::{hyp_run_events::HypRunEvent, hyp_run_state::HypRunState, single_hyp_status::SingleHypStatus, test_collection::TestCollection};


#[derive(Clone, Debug, PartialEq)]
pub struct BuildFailedTestRun
{
    pub message: String
}

#[derive(Clone, Debug, PartialEq)]
pub struct FailedTestRun
{
    pub inner_error_display: String
}

#[derive(Debug, Clone)]
pub struct TestRun
{
    pub state: HypRunState,
    pub tests: TestCollection
}

impl TestRun
{
    pub fn from_state(state: HypRunState) -> Self
    {
        Self {
            state,
            tests: TestCollection::default()
        }
    }

    pub fn from_events<TEvents>(events: TEvents) -> Self
    where 
        TEvents: IntoIterator<Item = HypRunEvent>
    {
        let mut test_run = Self::from_state(HypRunState::Idle);

        for event in events
        {
            test_run.update(event);
        }

        test_run
    }

    pub fn from_failed(failure: FailedTestRun) -> Self
    {
        Self::from_state(HypRunState::Failed(failure))
    }

    pub fn update(&mut self, event: HypRunEvent) -> bool
    {
        match event
        {
            HypRunEvent::Start =>
            {
                self.state = HypRunState::Running;
                for test in &mut self.tests
                {
                    test.status = SingleHypStatus::Unknown;
                    test.output.clear();
                }

                true
            }
            HypRunEvent::StartSingle { hyp, clear_tests } =>
            {
                if let Some(hyp) =
                {
                    if clear_tests
                    {
                        self.tests.clear_except(&hyp)
                    }
                    else 
                    {
                        self.tests.find_mut(&hyp)
                    }
                }
                {
                    self.state = HypRunState::Running;
                    hyp.status = SingleHypStatus::Unknown;
                    hyp.output.clear();

                    return true;
                }
                
                false
            }
            HypRunEvent::TestFinished(test) =>
            {
                self.state = HypRunState::Running;

                self.tests.add_or_update(test);

                true
            }
            HypRunEvent::NoTests =>
            {
                self.state = HypRunState::Idle;
                true
            }
            HypRunEvent::Compiling(message) =>
            {
                self.state = HypRunState::Building(message.clone());
                true
            }
            HypRunEvent::TestsCompleted =>
            {
                self.state = HypRunState::Idle;
                true
            }
            HypRunEvent::BuildError(message) =>
            {
                self.state = HypRunState::BuildFailed(BuildFailedTestRun { message });
                true
            }
            HypRunEvent::ErrorOutput { hyp, message } =>
            {
                if !message.is_empty()
                    && let Some(updated_test) = self.tests.find_mut(&hyp)
                {
                    updated_test.output.push(message);
                    return true;
                }

                false
            }
            HypRunEvent::HypRunError(message) =>
            {
                *self = TestRun::from_failed(message);
                true
            }
        }
    }
}

impl Default for TestRun
{
    fn default() -> Self
    {
        Self {
            state: HypRunState::Idle,
            tests: TestCollection::default()
        }
    }
}
