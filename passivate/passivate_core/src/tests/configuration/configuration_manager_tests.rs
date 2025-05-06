use passivate_delegation::{channel, Tx};

use crate::configuration::{ConfigurationManager, PassivateConfig};


#[test]
pub fn configuration_update_changes_configuration() {
    let configuration = PassivateConfig::default();
    let mut manager = ConfigurationManager::new(configuration, Tx::stub());

    manager.update(|c| {
        c.snapshots_path = Some(String::from("Example/path"));
    });

    assert_eq!(Some("Example/path"), manager.snapshots_path().as_deref());
}

#[test]
pub fn configuration_change_is_broadcast() {
    let configuration = PassivateConfig::default();
    let (tx, rx) = channel();
    let mut manager = ConfigurationManager::new(configuration, tx);

    let rx2 = rx.clone();

    manager.update(|c| {
        c.coverage_enabled = true;
    });

    let broadcast1 = rx.try_iter().last().unwrap();
    let broadcast2 = rx2.try_iter().last().unwrap();

    assert_eq!(broadcast1, broadcast2);
}