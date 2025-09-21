use crate::test_helpers::test_name::test_name;
use crate::test_run_model::{SingleTest, SingleTestStatus, TestId, TestRunEvent};

enum State
{
    Normal,
    ErrorOutput
}

pub struct NextestParser
{
    state: State,
    current_test: Option<TestId>
}

impl Default for NextestParser
{
    fn default() -> Self
    {
        Self {
            state: State::Normal,
            current_test: None
        }
    }
}

impl NextestParser
{
    pub fn parse_line(&mut self, line: &str) -> Option<TestRunEvent>
    {
        let trimmed = line.trim();

        if trimmed.starts_with("PASS")
        {
            self.state = State::Normal;
            let name = trimmed.split(" ").last().unwrap_or(trimmed);
            let name = name.split("::").last().unwrap_or(name);
            let test = SingleTest::new(name.to_string(), SingleTestStatus::Passed, vec![]);

            self.current_test = Some(test.id().clone());

            return Some(TestRunEvent::TestFinished(test));
        }
        else if trimmed.starts_with("FAIL")
        {
            self.state = State::Normal;
            let name = trimmed.split(" ").last().unwrap_or(trimmed);
            let name = name.split("::").last().unwrap_or(name);
            let test = SingleTest::new(name.to_string(), SingleTestStatus::Failed, vec![]);

            self.current_test = Some(test.id().clone());

            return Some(TestRunEvent::TestFinished(test));
        }
        else if trimmed.starts_with("Compiling")
        {
            return Some(TestRunEvent::Compiling(trimmed.to_string()));
        }
        else if trimmed.contains("stderr")
        {
            self.state = State::ErrorOutput;
        }
        else if trimmed.starts_with("error[")
        {
            return Some(TestRunEvent::BuildError(trimmed.to_string()));
        }
        else if trimmed.contains("────────────") || trimmed.starts_with("Summary")
        {
        }
        else if let (State::ErrorOutput, Some(current_test)) = (&self.state, &self.current_test)
        {
            return Some(TestRunEvent::ErrorOutput {
                test: current_test.clone(),
                message: trimmed.to_string()
            });
        }

        None
    }
}
