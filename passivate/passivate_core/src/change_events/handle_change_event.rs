use crate::change_events::change_event::ChangeEvent;

pub trait HandleChangeEvent {
    fn handle_event(&mut self, event: ChangeEvent);
}