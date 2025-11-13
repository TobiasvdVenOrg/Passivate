use passivate_model_session::hyp::Hyp;

pub enum PassivateStateChange<'a>
{
    HypSelected(&'a Hyp),
    HypDetailsChanged(&'a Hyp)
}
