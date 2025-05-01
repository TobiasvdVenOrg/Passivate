use crate::configuration::{ConfigurationManager, PassivateConfig};


#[test]
pub fn configuration_update_changes_configuration() {
    let configuration = PassivateConfig::default();
    let mut manager = ConfigurationManager::new(configuration);

    manager.update(|c| {
        c.snapshots_path = Some(String::from("Example/path"));
    });

    assert_eq!(Some("Example/path"), manager.snapshots_path().as_deref());
}
