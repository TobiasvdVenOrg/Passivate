use passivate_delegation::{Cancellation, Tx};

use super::TestRunError;
use crate::{passivate_nextest::NextestParser, test_execution::TestRunner, test_run_model::{TestId, TestRun, TestRunEvent, TestRunState}};

#[faux::create]
pub struct TestRunProcessor
{
    run_tests: TestRunner,
    parse_output: NextestParser,
    test_run: TestRun
}

#[faux::methods]
impl TestRunProcessor
{
    pub fn new(run_tests: TestRunner, parse_output: NextestParser) -> Self
    {
        Self::from_test_run(run_tests, parse_output, TestRun::default())
    }

    pub fn from_test_run(run_tests: TestRunner, parse_output: NextestParser, test_run: TestRun) -> Self
    {
        Self {
            run_tests,
            parse_output,
            test_run
        }
    }

    fn update(&mut self, event: TestRunEvent, sender: &mut Tx<TestRun>)
    {
        if self.test_run.update(event)
        {
            sender.send(self.test_run.clone());
        }
    }

    pub fn run_tests(&mut self, sender: &mut Tx<TestRun>, instrument_coverage: bool, cancellation: Cancellation) -> Result<(), TestRunError>
    {
        self.update(TestRunEvent::Start, sender);

        cancellation.check()?;

        self.run_tests.run_tests(instrument_coverage, cancellation.clone())?;

        cancellation.check()?;

        match self.test_run.state
        {
            TestRunState::BuildFailed(_) =>
            {}
            _ =>
            {
                if self.test_run.tests.is_empty()
                {
                    self.update(TestRunEvent::NoTests, sender);
                }
                else
                {
                    self.update(TestRunEvent::TestsCompleted, sender);
                }
            }
        }

        Ok(())
    }

    pub fn run_test(&mut self, sender: &mut Tx<TestRun>, id: &TestId, update_snapshots: bool, cancellation: Cancellation) -> Result<(), TestRunError>
    {
        if let Some(test) = self.test_run.tests.find(id)
        {
            self.update(
                TestRunEvent::StartSingle {
                    test: id.clone(),
                    clear_tests: !update_snapshots // if we're just updating a snapshot we don't need to clear the other tests
                },
                sender
            );

            self.run_tests.run_test(&test.name, update_snapshots, cancellation.clone())?;

            cancellation.check()?;
            
            self.update(TestRunEvent::TestsCompleted, sender);
        }

        Ok(())
    }
}
