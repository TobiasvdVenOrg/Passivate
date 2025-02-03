use crate::change_events::change_event::ChangeEvent;

pub trait HandleChangeEvent : Send {
    fn handle_event(&mut self, event: ChangeEvent);
}