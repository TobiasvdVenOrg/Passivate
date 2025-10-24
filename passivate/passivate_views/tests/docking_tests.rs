use std::fs;
use std::sync::Arc;

use camino::Utf8PathBuf;
use egui_kittest::Harness;
use galvanic_assert::assert_that;
use itertools::Itertools;
use passivate_configuration::configuration_errors::{ConfigurationLoadError, ConfigurationPersistError};
use passivate_configuration::configuration_source::ConfigurationSource;
use passivate_hyp_names::test_name;
use passivate_testing::path_resolution::{clean_directory, test_output_path};
use passivate_testing::spy_log::SpyLog;
use passivate_views::docking::docking_layout::{DockId, DockingLayout};
use passivate_views::docking::layout_management::LayoutManagement;
use passivate_views::docking::view::View;

#[test]
pub fn loading_a_default_layout_will_succeed_when_no_file_exists_and_later_create_the_file()
-> Result<(), ConfigurationLoadError>
{
    let dir = test_output_path().join(test_name!());
    clean_directory(&dir);
    let path = dir.join("does_not_exist_yet.toml");

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
pub fn placeholder_view_is_used_when_layout_contains_missing_ids()
{
    use egui_dock::TabViewer;

    let views: Vec<Box<dyn View>> = Vec::new();
    let mut tab_viewer = passivate_views::docking::dock_views::TabViewer::new(views.into_iter());

    let mut dock_id: DockId = "does not exist".into();

    let ui = |ui: &mut egui::Ui| {
        tab_viewer.ui(ui, &mut dock_id);
    };

    let mut harness = Harness::new_ui(ui);

    harness.run();
    harness.fit_contents();
    harness.snapshot(&test_name!());
}

#[test]
pub fn dock_state_can_be_reconstructed_from_serialized_form()
{
    let id_a: DockId = "view_a".into();
    let id_b: DockId = "view_b".into();

    let test_ids = vec![id_a.clone(), id_b.clone()];

    let dir = test_output_path().join(test_name!());
    clean_directory(&dir);
    let path = dir.join("docking_layout.toml");

    {
        let _layout = LayoutManagement::from_file_or_default(&path, || DockingLayout::new(test_ids.into_iter())).unwrap();   
    }

    let reloaded = LayoutManagement::from_file_or_default(&path, || panic!()).unwrap();

    assert_eq!(id_a, **reloaded.get_current().views().first().unwrap());
    assert_eq!(id_b, **reloaded.get_current().views().iter().next_back().unwrap());
}
