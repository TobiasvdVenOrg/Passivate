use crate::hyp::Hyp;

pub struct HypRun<'a>
{
    hyps: Vec<&'a Hyp>
}

impl<'a> HypRun<'a>
{
    pub fn new(hyps: impl IntoIterator<Item = &'a Hyp>) -> Self
    {
        Self {
            hyps: hyps.into_iter().collect()
        }
    }
}
