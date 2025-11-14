use std::ops::{Deref, DerefMut};

use passivate_configuration::configuration_manager::ConfigurationManager;
use passivate_delegation::{Rx, Tx};
use passivate_egui_docking::docking_layout::DockId;
use passivate_egui_docking::view::View;
use passivate_egui_view_configuration::ConfigurationView;
use passivate_egui_view_coverage::CoverageView;
use passivate_egui_view_details::DetailsView;
use passivate_egui_view_log::LogView;
use passivate_egui_view_session::SessionView;

pub enum PassivateView
{
    Configuration(ConfigurationDock),
    Coverage(CoverageDock),
    Details(DetailsDock),
    Log(LogDock),
    HypRun(SessionDock)
}

pub struct PassivateViews
{
    session_view: PassivateView,
    details_view: PassivateView,
    coverage_view: PassivateView,
    configuration_view: PassivateView,
    log_view: PassivateView
}

impl PassivateViews
{
    pub fn new(
        session_view: SessionView,
        details_view: DetailsView,
        coverage_view: CoverageView,
        configuration_view: ConfigurationView,
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

    pub fn stub() -> Self
    {
        let configuration = ConfigurationManager::default_config(Tx::stub());

        PassivateViews::new(
            SessionView,
            DetailsView::new(Tx::stub()),
            CoverageView::new(Rx::stub(), configuration.clone()),
            ConfigurationView::new(configuration, Tx::stub()),
            LogView::new(Rx::stub())
        )
    }

    pub fn get(&self) -> [&PassivateView; 5]
    {
        [
            &self.session_view,
            &self.details_view,
            &self.coverage_view,
            &self.configuration_view,
            &self.log_view
        ]
    }

    pub fn into(self) -> Vec<PassivateView>
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

    pub fn except_hyp_run_view(&self) -> [&PassivateView; 4]
    {
        [
            &self.details_view,
            &self.coverage_view,
            &self.configuration_view,
            &self.log_view
        ]
    }

    pub fn details_view(&self) -> &DetailsView
    {
        self.details_dock()
    }

    pub fn details_dock(&self) -> &DetailsDock
    {
        match &self.details_view
        {
            PassivateView::Details(details_view) => details_view,
            _ => panic!("expected details view")
        }
    }

    pub fn except_details_view(&self) -> [&PassivateView; 4]
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

    pub fn except_coverage_view(&self) -> [&PassivateView; 4]
    {
        [
            &self.session_view,
            &self.details_view,
            &self.configuration_view,
            &self.log_view
        ]
    }

    pub fn configuration_view(&self) -> &ConfigurationView
    {
        self.configuration_dock()
    }

    pub fn configuration_dock(&self) -> &ConfigurationDock
    {
        match &self.configuration_view
        {
            PassivateView::Configuration(configuration_view) => configuration_view,
            _ => panic!("expected configuration view")
        }
    }

    pub fn except_configuration_view(&self) -> [&PassivateView; 4]
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

    pub fn except_log_view(&self) -> [&PassivateView; 4]
    {
        [
            &self.session_view,
            &self.details_view,
            &self.coverage_view,
            &self.configuration_view
        ]
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

pub struct DetailsDock(DetailsView);

impl View for DetailsDock
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

impl Deref for DetailsDock
{
    type Target = DetailsView;

    fn deref(&self) -> &Self::Target
    {
        &self.0
    }
}

impl DerefMut for DetailsDock
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

pub struct ConfigurationDock(ConfigurationView);

impl View for ConfigurationDock
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

impl Deref for ConfigurationDock
{
    type Target = ConfigurationView;

    fn deref(&self) -> &Self::Target
    {
        &self.0
    }
}

impl DerefMut for ConfigurationDock
{
    fn deref_mut(&mut self) -> &mut Self::Target
    {
        &mut self.0
    }
}
