use crate::{configuration_view::ConfigurationView, coverage_view::CoverageView, details_view::DetailsView, docking::{docking_layout::DockId, view::View}, log_view::LogView, test_run_view::TestRunView};

pub enum PassivateView
{
    Configuration(ConfigurationView),
    Coverage(CoverageView),
    Details(DetailsView),
    Log(LogView),
    TestRun(TestRunView)
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
            PassivateView::TestRun(_) => DockId::from("test_run_view"),
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
            PassivateView::TestRun(_) => String::from("Tests"),
        }
    }
}
