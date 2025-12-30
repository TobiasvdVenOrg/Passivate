use std::ops::{Deref, DerefMut};

use passivate_configuration::configuration_manager::ConfigurationManager;
use passivate_delegation::{Rx, Tx};
use passivate_egui_docking::docking_layout::DockId;
use passivate_egui_docking::view::View;
use passivate_egui_view_configuration::ConfigurationView;
use passivate_egui_view_coverage::CoverageView;
use passivate_egui_view_details::details_view::DetailsView;
use passivate_egui_view_log::LogView;
use passivate_egui_view_session::session_view::SessionView;
use passivate_model_bridge::bridge::Bridge;
use passivate_model_bridge::hyp_run_bridge::{HypRunBridge, MockHypRunBridge};

pub enum PassivateView<TBridge: Bridge, THypRunBridge: HypRunBridge>
{
    Configuration(ConfigurationDock<THypRunBridge>),
    Coverage(CoverageDock),
    Details(DetailsDock<TBridge>),
    Log(LogDock),
    HypRun(SessionDock)
}

pub struct PassivateViews<TBridge: Bridge, THypRunBridge: HypRunBridge>
{
    session_view: PassivateView<TBridge, THypRunBridge>,
    details_view: PassivateView<TBridge, THypRunBridge>,
    coverage_view: PassivateView<TBridge, THypRunBridge>,
    configuration_view: PassivateView<TBridge, THypRunBridge>,
    log_view: PassivateView<TBridge, THypRunBridge>
}

impl<TBridge: Bridge, THypRunBridge: HypRunBridge> PassivateViews<TBridge, THypRunBridge>
{
    pub fn new(
        session_view: SessionView,
        details_view: DetailsView<TBridge>,
        coverage_view: CoverageView,
        configuration_view: ConfigurationView<THypRunBridge>,
        log_view: LogView
    ) -> Self
    {
        Self {
            session_view: PassivateView::HypRun(SessionDock(session_view)),
            details_view: PassivateView::Details(DetailsDock(details_view)),
            coverage_view: PassivateView::Coverage(CoverageDock(coverage_view)),
            configuration_view: PassivateView::Configuration(ConfigurationDock(configuration_view)),
            log_view: PassivateView::Log(LogDock(log_view))
        }
    }

    pub fn stub<TStubBridge: Bridge>() -> PassivateViews<TStubBridge, MockHypRunBridge>
    {
        let configuration = ConfigurationManager::default_config(Tx::stub());
        let hyp_runner = MockHypRunBridge::new();

        PassivateViews::new(
            SessionView,
            DetailsView::new(),
            CoverageView::new(Rx::stub(), configuration.clone()),
            ConfigurationView::new(configuration, hyp_runner),
            LogView::new(Rx::stub())
        )
    }

    pub fn get(&self) -> [&PassivateView<TBridge, THypRunBridge>; 5]
    {
        [
            &self.session_view,
            &self.details_view,
            &self.coverage_view,
            &self.configuration_view,
            &self.log_view
        ]
    }

    pub fn into(self) -> Vec<PassivateView<TBridge, THypRunBridge>>
    {
        [
            self.session_view,
            self.details_view,
            self.coverage_view,
            self.configuration_view,
            self.log_view
        ]
        .into_iter()
        .collect()
    }

    pub fn ids(&self) -> [DockId; 5]
    {
        self.get().map(|v| v.id())
    }

    pub fn session_view(&self) -> &SessionView
    {
        self.session_dock()
    }

    pub fn session_dock(&self) -> &SessionDock
    {
        match &self.session_view
        {
            PassivateView::HypRun(session_view) => session_view,
            _ => panic!("expected session view")
        }
    }

    pub fn except_hyp_run_view(&self) -> [&PassivateView<TBridge, THypRunBridge>; 4]
    {
        [
            &self.details_view,
            &self.coverage_view,
            &self.configuration_view,
            &self.log_view
        ]
    }

    pub fn details_view(&self) -> &DetailsView<TBridge>
    {
        self.details_dock()
    }

    pub fn details_dock(&self) -> &DetailsDock<TBridge>
    {
        match &self.details_view
        {
            PassivateView::Details(details_view) => details_view,
            _ => panic!("expected details view")
        }
    }

    pub fn except_details_view(&self) -> [&PassivateView<TBridge, THypRunBridge>; 4]
    {
        [
            &self.session_view,
            &self.coverage_view,
            &self.configuration_view,
            &self.log_view
        ]
    }

    pub fn coverage_view(&self) -> &CoverageView
    {
        self.coverage_dock()
    }

    pub fn coverage_dock(&self) -> &CoverageDock
    {
        match &self.coverage_view
        {
            PassivateView::Coverage(coverage_view) => coverage_view,
            _ => panic!("expected coverage view")
        }
    }

    pub fn except_coverage_view(&self) -> [&PassivateView<TBridge, THypRunBridge>; 4]
    {
        [
            &self.session_view,
            &self.details_view,
            &self.configuration_view,
            &self.log_view
        ]
    }

    pub fn configuration_view(&self) -> &ConfigurationView<THypRunBridge>
    {
        self.configuration_dock()
    }

    pub fn configuration_dock(&self) -> &ConfigurationDock<THypRunBridge>
    {
        match &self.configuration_view
        {
            PassivateView::Configuration(configuration_view) => configuration_view,
            _ => panic!("expected configuration view")
        }
    }

    pub fn except_configuration_view(&self) -> [&PassivateView<TBridge, THypRunBridge>; 4]
    {
        [&self.session_view, &self.details_view, &self.coverage_view, &self.log_view]
    }

    pub fn log_view(&self) -> &LogView
    {
        self.log_dock()
    }

    pub fn log_dock(&self) -> &LogDock
    {
        match &self.log_view
        {
            PassivateView::Log(log_view) => log_view,
            _ => panic!("expected log view")
        }
    }

    pub fn except_log_view(&self) -> [&PassivateView<TBridge, THypRunBridge>; 4]
    {
        [
            &self.session_view,
            &self.details_view,
            &self.coverage_view,
            &self.configuration_view
        ]
    }
}

impl<TBridge: Bridge, THypRunBridge: HypRunBridge> View for PassivateView<TBridge, THypRunBridge>
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

pub struct SessionDock(SessionView);

impl View for SessionDock
{
    fn id(&self) -> DockId
    {
        DockId::from("session_view")
    }

    fn title(&self) -> String
    {
        String::from("Tests")
    }
}

impl Deref for SessionDock
{
    type Target = SessionView;

    fn deref(&self) -> &Self::Target
    {
        &self.0
    }
}

impl DerefMut for SessionDock
{
    fn deref_mut(&mut self) -> &mut Self::Target
    {
        &mut self.0
    }
}

pub struct LogDock(LogView);

impl View for LogDock
{
    fn id(&self) -> DockId
    {
        DockId::from("log_view")
    }

    fn title(&self) -> String
    {
        String::from("Log")
    }
}

impl Deref for LogDock
{
    type Target = LogView;

    fn deref(&self) -> &Self::Target
    {
        &self.0
    }
}

impl DerefMut for LogDock
{
    fn deref_mut(&mut self) -> &mut Self::Target
    {
        &mut self.0
    }
}

pub struct DetailsDock<TBridge: Bridge>(DetailsView<TBridge>);

impl<TBridge: Bridge> View for DetailsDock<TBridge>
{
    fn id(&self) -> DockId
    {
        DockId::from("details_view")
    }

    fn title(&self) -> String
    {
        String::from("Details")
    }
}

impl<TBridge: Bridge> Deref for DetailsDock<TBridge>
{
    type Target = DetailsView<TBridge>;

    fn deref(&self) -> &Self::Target
    {
        &self.0
    }
}

impl<TBridge: Bridge> DerefMut for DetailsDock<TBridge>
{
    fn deref_mut(&mut self) -> &mut Self::Target
    {
        &mut self.0
    }
}

pub struct CoverageDock(CoverageView);

impl View for CoverageDock
{
    fn id(&self) -> DockId
    {
        DockId::from("coverage_view")
    }

    fn title(&self) -> String
    {
        String::from("Coverage")
    }
}

impl Deref for CoverageDock
{
    type Target = CoverageView;

    fn deref(&self) -> &Self::Target
    {
        &self.0
    }
}

impl DerefMut for CoverageDock
{
    fn deref_mut(&mut self) -> &mut Self::Target
    {
        &mut self.0
    }
}

pub struct ConfigurationDock<THypRunBridge: HypRunBridge>(ConfigurationView<THypRunBridge>);

impl<THypRunBridge: HypRunBridge> View for ConfigurationDock<THypRunBridge>
{
    fn id(&self) -> DockId
    {
        DockId::from("configuration_view")
    }

    fn title(&self) -> String
    {
        String::from("Configuration")
    }
}

impl<THypRunBridge: HypRunBridge> Deref for ConfigurationDock<THypRunBridge>
{
    type Target = ConfigurationView<THypRunBridge>;

    fn deref(&self) -> &Self::Target
    {
        &self.0
    }
}

impl<THypRunBridge: HypRunBridge> DerefMut for ConfigurationDock<THypRunBridge>
{
    fn deref_mut(&mut self) -> &mut Self::Target
    {
        &mut self.0
    }
}
