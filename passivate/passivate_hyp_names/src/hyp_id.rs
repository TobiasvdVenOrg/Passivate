
pub struct HypId
{
    
}

#[cfg(test)]
mod tests
{
    use crate::test_id;

    #[test]
    pub fn example_unit_test_id()
    {
        let id = test_id!();

        assert_eq!("passivate_hyp_names::hyp_id::tests::example_unit_test_id", id);
    }
}
