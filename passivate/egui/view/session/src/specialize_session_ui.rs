use egui::Ui;

pub trait SpecializeSessionUi
{
    fn ui(&self, ui: &mut Ui);
}

impl<T> SpecializeSessionUi for T
{
    default fn ui(&self, _: &mut Ui)
    {
        panic!(
            "missing UI specialization for session view and type {:?}",
            std::any::type_name::<T>()
        );
    }
}
