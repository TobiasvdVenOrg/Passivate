
use passivate_hyp_names::test_name;

mod sub_dir
{
    mod more_integration_tests;
}

#[test]
pub fn example_integration_test_name()
{
    let name = test_name!();

    assert_eq!("passivate_hyp_names::integration_tests::example_integration_test_name", name);
}

mod sub_mod
{
    use passivate_hyp_names::test_name;

    #[test]
    pub fn example_integration_test_in_sub_mod_name()
    {
        let name = test_name!();

        assert_eq!("passivate_hyp_names::integration_tests::sub_mod::example_integration_test_in_sub_mod_name", name);
    }
}
