use std::collections::hash_map::Entry;

use crate::hyp_run::HypRun;
use crate::hyp_run_events::{HypRunChange, HypRunEvent};
use crate::hyp_session_state::HypSessionState;
use crate::single_hyp_status::SingleHypStatus;

#[derive(Debug, Clone, Default)]
pub struct HypSession
{
    last_run: HypRun,
    current_run: HypRun,
    pub state: HypSessionState
}

impl HypSession
{
    pub fn new(last_run: HypRun, current_run: HypRun) -> Self
    {
        Self {
            last_run,
            current_run,
            state: HypSessionState::default()
        }
    }

    pub fn from_events<TEvents>(events: TEvents) -> Self
    where
        TEvents: IntoIterator<Item = HypRunEvent>
    {
        let mut session = Self::default();
        session.update_all(events);
        session
    }

    pub fn last_run(&self) -> &HypRun
    {
        &self.last_run
    }

    pub fn current_run(&self) -> &HypRun
    {
        &self.current_run
    }

    pub fn no_tests(&self) -> bool
    {
        self.last_run().hyps.is_empty() && self.current_run().hyps.is_empty()
    }

    pub fn update_all<TEvents>(&mut self, events: TEvents)
    where
        TEvents: IntoIterator<Item = HypRunEvent>
    {
        for event in events
        {
            self.update(event);
        }
    }

    pub fn update(&mut self, event: HypRunEvent) -> Option<HypRunChange<'_>>
    {
        let mut change = None;

        match event
        {
            HypRunEvent::Start =>
            {
                self.state = HypSessionState::Running;
                for test in &mut self.current_run.hyps.values_mut()
                {
                    test.status = SingleHypStatus::Unknown;
                    test.output.clear();
                }
            }
            HypRunEvent::StartSingle { hyp, clear_tests } =>
            {
                if let Some(hyp) = {
                    if clear_tests
                    {
                        self.current_run.hyps.retain(|id, _| *id == hyp);
                    }

                    self.current_run.hyps.get_mut(&hyp)
                }
                {
                    self.state = HypSessionState::Running;
                    hyp.status = SingleHypStatus::Unknown;
                    hyp.output.clear();
                }
            }
            HypRunEvent::TestFinished(hyp) =>
            {
                self.state = HypSessionState::Running;

                let inserted = match self.current_run.hyps.entry(hyp.id.clone())
                {
                    Entry::Occupied(mut occupied_entry) =>
                    {
                        occupied_entry.insert(hyp);
                        occupied_entry.into_mut()
                    }
                    Entry::Vacant(vacant_entry) => vacant_entry.insert(hyp)
                };

                change = Some(HypRunChange::HypUpdated(inserted));
            }
            HypRunEvent::NoTests =>
            {
                self.state = HypSessionState::Idle;
            }
            HypRunEvent::Compiling(message) =>
            {
                self.state = HypSessionState::Building(message.clone());
            }
            HypRunEvent::TestsCompleted =>
            {
                self.state = HypSessionState::Idle;
            }
            HypRunEvent::BuildError(message) =>
            {
                self.state = HypSessionState::BuildFailed(message);
            }
            HypRunEvent::ErrorOutput { hyp, message } =>
            {
                self.current_run.hyps.entry(hyp).and_modify(|h| h.output.push(message));
            }
            HypRunEvent::HypRunError(message) =>
            {
                self.state = HypSessionState::Failed(message);
            }
        }

        change
    }
}
