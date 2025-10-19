use std::fs;
use std::sync::Arc;

use camino::Utf8PathBuf;
use galvanic_assert::assert_that;
use itertools::Itertools;
use passivate_configuration::configuration_errors::{ConfigurationLoadError, ConfigurationPersistError};
use passivate_configuration::configuration_source::ConfigurationSource;
use passivate_hyp_names::test_name;
use passivate_testing::path_resolution::test_output_path;
use passivate_testing::spy_log::SpyLog;
use passivate_views::docking::docking_layout::{DockId, DockingLayout};
use passivate_views::docking::layout_management::LayoutManagement;
use passivate_views::docking::view::View;

struct TestView
{
    id: DockId
}

impl View for TestView
{
    fn id(&self) -> DockId
    {
        self.id.clone()
    }

    fn ui(&mut self, _ui: &mut egui_dock::egui::Ui)
    {
        todo!()
    }

    fn title(&self) -> String
    {
        todo!()
    }
}

#[test]
pub fn loading_a_default_layout_will_succeed_when_no_file_exists_and_later_create_the_file()
-> Result<(), ConfigurationLoadError>
{
    let path = test_output_path().join(test_name!()).join("does_not_exist_yet.toml");

    {
        let _layout = LayoutManagement::from_file_or_default(&path, || DockingLayout::new(vec![].into_iter()))?;
    }

    assert!(fs::exists(path).map_err(Arc::new)?);

    Ok(())
}

#[test]
pub fn failing_to_persist_layout_logs_an_error()
{
    let spy_log = SpyLog::set();

    {
        let mut source = ConfigurationSource::faux();
        source._when_load().then_return(Ok(None));
        source._when_persist().then_return(Err(ConfigurationPersistError::Path(Utf8PathBuf::new())));

        let _layout = LayoutManagement::from_source_or_default(source, || DockingLayout::new(vec![].into_iter())).unwrap();
    }

    let error = spy_log.into_iter().exactly_one().unwrap();

    assert_that!(error.starts_with("ERROR"));
}

#[test]
pub fn loading_a_specific_layout_will_fail_when_no_file_exists() {}

#[test]
pub fn dock_state_can_be_reconstructed_from_serialized_form()
{
    let id_a: DockId = "view_a".into();
    let id_b: DockId = "view_b".into();

    let test_views = vec![TestView { id: id_a.clone() }, TestView { id: id_b.clone() }]
        .into_iter()
        .map(|view| Box::new(view) as Box<dyn View>);

    let dock_state = DockingLayout::new(test_views.map(|view| view.id()));

    let path = test_output_path().join(test_name!()).join("docking_layout.toml");

    let source = ConfigurationSource::from_file(path);
    source.persist(&dock_state).unwrap();

    let reloaded = source.load().unwrap().unwrap();

    assert_eq!(id_a, **reloaded.views().first().unwrap());
    assert_eq!(id_b, **reloaded.views().iter().next_back().unwrap());
}
