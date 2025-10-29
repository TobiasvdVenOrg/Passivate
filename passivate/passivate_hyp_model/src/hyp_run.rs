use std::collections::{HashMap, hash_map::Entry};

use passivate_hyp_names::hyp_id::HypId;

use crate::{hyp_run_events::{HypRunChange, HypRunEvent}, hyp_run_state::HypRunState, single_hyp::SingleHyp, single_hyp_status::SingleHypStatus};

#[derive(Debug, Clone)]
pub struct HypRun
{
    pub state: HypRunState,
    pub hyps: HashMap<HypId, SingleHyp>
}

impl HypRun
{
    pub fn from_state(state: HypRunState) -> Self
    {
        Self {
            state,
            hyps: HashMap::default()
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

    pub fn update(&mut self, event: HypRunEvent) -> Option<HypRunChange<'_>>
    {
        let mut change = None;

        match event
        {
            HypRunEvent::Start =>
            {
                self.state = HypRunState::Running;
                for test in &mut self.hyps.values_mut()
                {
                    test.status = SingleHypStatus::Unknown;
                    test.output.clear();
                }
            }
            HypRunEvent::StartSingle { hyp, clear_tests } =>
            {
                if let Some(hyp) =
                {
                    if clear_tests
                    {
                        self.hyps.retain(|id, _| 
                        {
                            *id == hyp
                        });
                    }

                    self.hyps.get_mut(&hyp)
                }
                {
                    self.state = HypRunState::Running;
                    hyp.status = SingleHypStatus::Unknown;
                    hyp.output.clear();
                }
            }
            HypRunEvent::TestFinished(hyp) =>
            {
                self.state = HypRunState::Running;

                let inserted = match self.hyps.entry(hyp.id.clone())
                {
                    Entry::Occupied(mut occupied_entry) => {
                        occupied_entry.insert(hyp);
                        occupied_entry.into_mut()
                    },
                    Entry::Vacant(vacant_entry) => vacant_entry.insert(hyp),
                };
                
                change = Some(HypRunChange::HypUpdated(inserted));
            }
            HypRunEvent::NoTests =>
            {
                self.state = HypRunState::Idle;
            }
            HypRunEvent::Compiling(message) =>
            {
                self.state = HypRunState::Building(message.clone());
            }
            HypRunEvent::TestsCompleted =>
            {
                self.state = HypRunState::Idle;
            }
            HypRunEvent::BuildError(message) =>
            {
                self.state = HypRunState::BuildFailed(message);
            }
            HypRunEvent::ErrorOutput { hyp, message } =>
            {
                self.hyps.entry(hyp).and_modify(|h| h.output.push(message));
            }
            HypRunEvent::HypRunError(message) =>
            {
                self.state = HypRunState::Failed(message);
            }
        }

        change
    }

    pub fn add_hyp(&mut self, hyp: SingleHyp) -> Option<SingleHyp>
    {
        self.hyps.insert(hyp.id.clone(), hyp)
    }
}

impl Default for HypRun
{
    fn default() -> Self
    {
        Self {
            state: HypRunState::Idle,
            hyps: HashMap::default()
        }
    }
}
