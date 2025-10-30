mod helpers;

use std::fs;
use std::sync::Arc;

use camino::Utf8PathBuf;
use egui_dock::TabViewer;
use egui_kittest::Harness;
use galvanic_assert::assert_that;
use itertools::Itertools;
use passivate_configuration::configuration_errors::{ConfigurationLoadError, ConfigurationPersistError};
use passivate_configuration::configuration_source::ConfigurationSource;
use passivate_hyp_names::test_name;
use passivate_testing::path_resolution::{clean_directory, test_output_path};
use passivate_testing::spy_log::SpyLog;
use passivate_egui::docking::dock_views::{DockViewer, DockViews};
use passivate_egui::docking::docking_layout::{DockId, DockingLayout};
use passivate_egui::docking::layout_management::LayoutManagement;
use passivate_egui::docking::view::View;

#[test]
pub fn loading_a_default_layout_will_succeed_when_no_file_exists_and_later_create_the_file()
-> Result<(), ConfigurationLoadError>
{
    let dir = test_output_path().join(test_name!());
    clean_directory(&dir);
    let path = dir.join("does_not_exist_yet.toml");

    {
        let _layout = LayoutManagement::from_file_or_default(&path, || DockingLayout::new(vec![]))?;
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
        source
            ._when_persist()
            .then_return(Err(ConfigurationPersistError::Path(Utf8PathBuf::new())));

        let _layout = LayoutManagement::from_source_or_default(source, || DockingLayout::new(vec![])).unwrap();
    }

    let error = spy_log.into_iter().exactly_one().unwrap();

    assert_that!(error.starts_with("ERROR"));
}

struct TestView;

impl View for TestView
{
    fn id(&self) -> DockId
    {
        todo!()
    }

    fn title(&self) -> String
    {
        todo!()
    }
}

#[test]
pub fn placeholder_view_is_used_when_layout_contains_missing_ids()
{
    let mut dock_views = DockViews::new(Vec::new());
    let mut context = ();
    let custom_ui = |_ui: &mut egui::Ui, _view: &mut TestView, _state: &mut ()| panic!();

    let mut dock_viewer = DockViewer {
        dock_views: &mut dock_views,
        context: &mut context,
        custom_ui
    };

    let mut dock_id: DockId = "does not exist".into();

    let ui = |ui: &mut egui::Ui| {
        dock_viewer.ui(ui, &mut dock_id);
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
        let _layout = LayoutManagement::from_file_or_default(&path, || DockingLayout::new(test_ids)).unwrap();
    }

    let mut reloaded = LayoutManagement::from_file_or_default(&path, || panic!()).unwrap();

    assert_eq!(id_a, **reloaded.get_current().views().first().unwrap());
    assert_eq!(id_b, **reloaded.get_current().views().iter().next_back().unwrap());
}
