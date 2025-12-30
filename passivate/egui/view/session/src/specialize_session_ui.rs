use egui::Ui;

use crate::session_view::HypUiAction;

pub trait SpecializeSessionUi
{
    fn ui(&self, ui: &mut Ui) -> Option<HypUiAction>;
}

impl<T> SpecializeSessionUi for T
{
    default fn ui(&self, _: &mut Ui) -> Option<HypUiAction>
    {
        panic!(
            "missing UI specialization for session view and type {:?}",
            std::any::type_name::<T>()
        );
    }
}
