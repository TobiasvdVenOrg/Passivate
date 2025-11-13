use passivate_egui_docking::docking_layout::DockId;
use passivate_egui_docking::view::View;

use crate::views::*;

impl View for TestRunView
{
    fn id(&self) -> DockId
    {
        DockId::from("hyp_run_view")
    }

    fn title(&self) -> String
    {
        String::from("Tests")
    }
}

impl View for LogView
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

impl View for DetailsView
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

impl View for CoverageView
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

impl View for ConfigurationView
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
