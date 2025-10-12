
use passivate_configuration::configuration::Configuration;
use passivate_delegation::Tx;

use crate::configuration::ConfigurationManager;

#[test]
pub fn configuration_update_changes_configuration()
{
    let configuration = Configuration::default();
    let mut manager = ConfigurationManager::new(configuration, Tx::stub());

    manager.update(|c| {
        c.snapshots_path = Some(String::from("Example/path"));
    });

    let snapshots_path = manager.get(|c| c.snapshots_path.clone());

    assert_eq!(Some("Example/path"), snapshots_path.as_deref());
}

#[test]
pub fn configuration_change_is_broadcast()
{
    let configuration = Configuration::default();
    let (tx, rx1, rx2) = Tx::multi_2();
    let mut manager = ConfigurationManager::new(configuration, tx);

    manager.update(|c| {
        c.coverage_enabled = true;
    });

    let broadcast1 = rx1.last().unwrap();
    let broadcast2 = rx2.last().unwrap();

    assert_eq!(broadcast1, broadcast2);
}
