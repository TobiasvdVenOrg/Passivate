
use passivate_hyp_names::test_id;

mod sub_dir
{
    mod more_integration_tests;
}

#[test]
pub fn example_integration_test_id()
{
    let id = test_id!().get_fully_qualified("::");

    assert_eq!("passivate_hyp_names::integration_tests::example_integration_test_id", id);
}

mod sub_mod
{
    use passivate_hyp_names::test_id;

    #[test]
    pub fn example_integration_test_in_sub_mod_id()
    {
        let id = test_id!().get_fully_qualified("::");

        assert_eq!("passivate_hyp_names::integration_tests::sub_mod::example_integration_test_in_sub_mod_id", id);
    }
}
