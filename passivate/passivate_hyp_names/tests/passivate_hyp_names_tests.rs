use passivate_hyp_names::test_id;


#[test]
pub fn example_integration_test_id_with_package_name_contained()
{
    let id = test_id!().get_fully_qualified("::");

    assert_eq!("passivate_hyp_names::passivate_hyp_names_tests::example_integration_test_id_with_package_name_contained", id);
}
