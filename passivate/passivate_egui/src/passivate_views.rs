use passivate_configuration::configuration_manager::ConfigurationManager;
use passivate_delegation::{Rx, Tx};

use crate::docking::docking_layout::DockId;
use crate::docking::view::View;
use crate::views::*;

pub enum PassivateView
{
    Configuration(ConfigurationView),
    Coverage(CoverageView),
    Details(DetailsView),
    Log(LogView),
    HypRun(TestRunView)
}

pub struct PassivateViews
{
    tests_view: PassivateView,
    details_view: PassivateView,
    coverage_view: PassivateView,
    configuration_view: PassivateView,
    log_view: PassivateView
}

impl PassivateViews
{
    pub fn new(
        tests_view: TestRunView,
        details_view: DetailsView,
        coverage_view: CoverageView,
        configuration_view: ConfigurationView,
        log_view: LogView
    ) -> Self
    {
        Self {
            tests_view: PassivateView::HypRun(tests_view),
            details_view: PassivateView::Details(details_view),
            coverage_view: PassivateView::Coverage(coverage_view),
            configuration_view: PassivateView::Configuration(configuration_view),
            log_view: PassivateView::Log(log_view)
        }
    }

    pub fn stub() -> Self
    {
        let configuration = ConfigurationManager::default_config(Tx::stub());

        PassivateViews::new(
            TestRunView, 
            DetailsView::new(Tx::stub()), 
            CoverageView::new(Rx::stub(), configuration.clone()), 
            ConfigurationView::new(configuration, Tx::stub()),
            LogView::new(Rx::stub()))
    }

    pub fn get(&self) -> [&PassivateView; 5]
    {
        [
            &self.tests_view,
            &self.details_view,
            &self.coverage_view,
            &self.configuration_view,
            &self.log_view
        ]
    }

    pub fn into(self) -> Vec<PassivateView>
    {
        [
            self.tests_view,
            self.details_view,
            self.coverage_view,
            self.configuration_view,
            self.log_view
        ]
        .into_iter().collect()
    }

    pub fn ids(&self) -> [DockId; 5]
    {
        self.get().map(|v| v.id())
    }

    pub fn hyp_run_view(&self) -> &TestRunView
    {
        match &self.tests_view
        {
            PassivateView::HypRun(test_run_view) => test_run_view,
            _ => panic!("expected hyp run view")
        }
    }

    pub fn details_view(&self) -> &DetailsView
    {
        match &self.details_view
        {
            PassivateView::Details(details_view) => details_view,
            _ => panic!("expected details view")
        }
    }

    pub fn coverage_view(&self) -> &CoverageView
    {
        match &self.coverage_view
        {
            PassivateView::Coverage(coverage_view) => coverage_view,
            _ => panic!("expected coverage view")
        }
    }

    pub fn configuration_view(&self) -> &ConfigurationView
    {
        match &self.configuration_view
        {
            PassivateView::Configuration(configuration_view) => configuration_view,
            _ => panic!("expected configuration view")
        }
    }

    pub fn log_view(&self) -> &LogView
    {
        match &self.log_view
        {
            PassivateView::Log(log_view) => log_view,
            _ => panic!("expected log view")
        }
    }
}

impl View for PassivateView
{
    fn id(&self) -> DockId
    {
        match self
        {
            PassivateView::Configuration(v) => v.id(),
            PassivateView::Coverage(v) => v.id(),
            PassivateView::Details(v) => v.id(),
            PassivateView::Log(v) => v.id(),
            PassivateView::HypRun(v) => v.id()
        }
    }

    fn title(&self) -> String
    {
        match self
        {
            PassivateView::Configuration(v) => v.title(),
            PassivateView::Coverage(v) => v.title(),
            PassivateView::Details(v) => v.title(),
            PassivateView::Log(v) => v.title(),
            PassivateView::HypRun(v) => v.title()
        }
    }
}
