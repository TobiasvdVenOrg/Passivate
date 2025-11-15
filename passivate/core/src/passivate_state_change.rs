use passivate_model_core::hyp::Hyp;

pub enum PassivateStateChange<'a>
{
    HypSelected(&'a Hyp),
    HypDetailsChanged(&'a Hyp)
}
