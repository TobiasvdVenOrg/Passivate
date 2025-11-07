use passivate_hyp_model::hyp::Hyp;

pub enum PassivateStateChange<'a>
{
    HypSelected(&'a Hyp),
    HypDetailsChanged(&'a Hyp)
}
