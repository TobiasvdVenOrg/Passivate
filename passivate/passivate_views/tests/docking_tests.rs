use passivate_configuration::configuration_source::ConfigurationSource;
use passivate_hyp_names::test_name;
use passivate_testing::path_resolution::test_output_path;
use passivate_views::docking::{docking_layout::{DockId, DockingLayout}, view::View};

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
pub fn dock_state_can_be_reconstructed_from_serialized_form() 
{
    let id_a: DockId = "view_a".into();
    let id_b: DockId = "view_b".into();

    let test_views = vec![ 
        TestView { id: id_a.clone() }, 
        TestView { id: id_b.clone() } ]
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
