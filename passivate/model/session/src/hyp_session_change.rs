use crate::hyp::Hyp;

pub enum HypSessionChange<'a>
{
    HypUpdated(&'a Hyp)
}
