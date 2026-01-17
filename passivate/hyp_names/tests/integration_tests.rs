use passivate_hyp_names::hyp_name_strategy::HypNameStrategy;
use passivate_hyp_names::test_id;

mod sub_dir
{
    mod more_integration_tests;
}

#[test]
pub fn example_integration_test_id()
{
    let id = test_id!();
    let fully_qualified = id.fully_qualified("::");

    assert_eq!(
        "passivate_hyp_names::integration_tests::example_integration_test_id",
        fully_qualified
    );
}

#[test]
pub fn example_integration_test_id_qualified_without_crate()
{
    let id = test_id!();
    let name = id.name(HypNameStrategy::QualifiedWithoutCrate {
        separator: String::from("::")
    });

    assert_eq!("example_integration_test_id_qualified_without_crate", name);
}

#[test]
pub fn get_package_and_crate_name_from_test_id()
{
    let id = test_id!();

    assert_eq!(
        "passivate_hyp_names::integration_tests",
        id.package_crate_name().join("::").as_str()
    );
}

mod sub_mod
{
    use passivate_hyp_names::test_id;

    #[test]
    pub fn example_integration_test_in_sub_mod_id()
    {
        let id = test_id!().fully_qualified("::");

        assert_eq!(
            "passivate_hyp_names::integration_tests::sub_mod::example_integration_test_in_sub_mod_id",
            id
        );
    }
}
