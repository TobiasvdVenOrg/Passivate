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
            PassivateView::Configuration(configuration_view) => todo!(),
            PassivateView::Coverage(coverage_view) => todo!(),
            PassivateView::Details(details_view) => todo!(),
            PassivateView::Log(log_view) => todo!(),
            PassivateView::TestRun(test_run_view) => todo!(),
        }
    }

    fn title(&self) -> String
    {
        match self
        {
            PassivateView::Configuration(configuration_view) => todo!(),
            PassivateView::Coverage(coverage_view) => todo!(),
            PassivateView::Details(details_view) => todo!(),
            PassivateView::Log(log_view) => todo!(),
            PassivateView::TestRun(test_run_view) => todo!(),
        }
    }
}
