
use passivate_hyp_names::test_id;

#[test]
pub fn example_integration_test_in_sub_dir_id()
{
    let id = test_id!();

    assert_eq!("passivate_hyp_names::integration_tests::sub_dir::more_integration_tests::example_integration_test_in_sub_dir_id", id);
}

mod sub_mod
{
    use passivate_hyp_names::test_id;

    #[test]
    pub fn example_integration_test_in_sub_dir_and_mod_id()
    {
        let id = test_id!();

        assert_eq!("passivate_hyp_names::integration_tests::sub_dir::more_integration_tests::sub_mod::example_integration_test_in_sub_dir_and_mod_id", id);
    }
}
