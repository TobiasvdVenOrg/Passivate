use crate::docking::docking_layout::DockId;

pub trait View
{
    fn id(&self) -> DockId;
    fn title(&self) -> String;
}
