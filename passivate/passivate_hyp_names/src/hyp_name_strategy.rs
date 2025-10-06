#[cfg(test)]
mod tests
{
    use crate::test_name;

    #[test]
    pub fn example_unit_test_name()
    {
        let name = test_name!();

        assert_eq!("passivate_hyp_names::hyp_name_strategy::tests::example_unit_test_name", name);
    }
}
