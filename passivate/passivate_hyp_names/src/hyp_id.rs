use thiserror::Error;

// pub enum HypNameStrategy
// {
//     Default,
//     NameOnly
// }

// impl HypNameStrategy
// {
//     fn convert<'a>(&self, id: &HypId) -> &'a str
//     {
//         match self
//         {
//             HypNameStrategy::Default | HypNameStrategy::NameOnly => id.full_id.split_terminator("::").last()
//         }
//     }
// }

pub struct HypId
{
    parts: Vec<String>
}

impl HypId
{
    // pub fn new(test_id: String) -> Self
    // {
    //     Self { full_id: test_id }
    // }

    // pub fn into_name<'a>(&self, name_only: &HypNameStrategy) -> &'a str
    // {
    //     name_only.convert(&self.full_id)
    // }
}

#[derive(Error, Debug, PartialEq)]
pub enum HypIdError
{
    #[error("hyp id was not in a valid format: {0}")]
    InvalidFormat(String)
}

impl TryFrom<&str> for HypId
{
    type Error = HypIdError;

    fn try_from(value: &str) -> Result<Self, Self::Error>
    {
        let mut parts: Vec<String> = value.split_terminator("::").map(String::from).collect();

        if parts.len() < 2
        {
            return Err(HypIdError::InvalidFormat(parts.remove(0)))
        }

        Ok(Self { parts })
    }
}

#[cfg(test)]
mod tests
{
    use crate::hyp_id::{HypId, HypIdError};
    use crate::test_id;
    use rstest::*;
    use galvanic_assert::*;
    use galvanic_assert::matchers::*;

    #[test]
    pub fn example_unit_test_id()
    {
        let id = test_id!();

        assert_eq!("passivate_hyp_names::hyp_id::tests::example_unit_test_id", id);
    }

    #[rstest]
    #[case::no_separators("test_name")]
    pub fn invalid_test_id_error(#[case] test_id: &str)
    {
        let id: Result<HypId, HypIdError> = test_id.try_into();

        assert!(matches!(id, Err(HypIdError::InvalidFormat(_))));
    }

    // #[test]
    // pub fn id_as_name_only_from_unit_test()
    // {
    //     let id = HypId::new(test_id!());

    //     let name = id.into_name(&HypNameStrategy::NameOnly);

    //     assert_eq!("id_as_name_only_from_unit_test", name);
    // }
}
