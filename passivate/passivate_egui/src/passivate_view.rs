use crate::{docking::{docking_layout::DockId, view::View}, views::*};

pub enum PassivateView
{
    Configuration(ConfigurationView),
    Coverage(CoverageView),
    Details(DetailsView),
    Log(LogView),
    HypRun(TestRunView)
}

impl View for PassivateView
{
    fn id(&self) -> DockId
    {
        match self
        {
            PassivateView::Configuration(_) => DockId::from("configuration_view"),
            PassivateView::Coverage(_) => DockId::from("coverage_view"),
            PassivateView::Details(_) => DockId::from("details_view"),
            PassivateView::Log(_) => DockId::from("log_view"),
            PassivateView::HypRun(_) => DockId::from("test_run_view"),
        }
    }

    fn title(&self) -> String
    {
        match self
        {
            PassivateView::Configuration(_) => String::from("Configuration"),
            PassivateView::Coverage(_) => String::from("Coverage"),
            PassivateView::Details(_) => String::from("Details"),
            PassivateView::Log(_) => String::from("Log"),
            PassivateView::HypRun(_) => String::from("Tests"),
        }
    }
}
