use passivate_hyp_model::single_hyp::SingleHyp;


pub enum PassivateStateChange<'a>
{
    HypSelected(&'a SingleHyp),
    HypDetailsChanged(&'a SingleHyp)    
}
