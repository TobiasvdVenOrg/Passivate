use camino::Utf8Path;
use passivate_configuration::configuration_errors::ConfigurationLoadError;

use crate::configuration_view::ConfigurationView;
use crate::coverage_view::CoverageView;
use crate::details_view::DetailsView;
use crate::docking::docking_layout::DockingLayout;
use crate::docking::layout_management::LayoutManagement;
use crate::docking::view::View;
use crate::log_view::LogView;
use crate::test_run_view::TestRunView;

pub fn load(
    path: &Utf8Path,
    test_run_view: &TestRunView,
    details_view: &DetailsView,
    coverage_view: &CoverageView,
    configuration_view: &ConfigurationView,
    log_view: &LogView
) -> Result<LayoutManagement, ConfigurationLoadError>
{
    LayoutManagement::from_file_or_default(path, ||
    {
        let views: Vec<Box<&dyn View>> = vec![
            Box::new(test_run_view),
            Box::new(details_view),
            Box::new(coverage_view),
            Box::new(configuration_view),
            Box::new(log_view),
        ];

        DockingLayout::new(views.iter().map(|view| view.id()))
    })
}
