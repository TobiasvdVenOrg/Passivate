use passivate_hyp_model::single_hyp::SingleHyp;


pub enum PassivateStateChange<'a>
{
    HypDetailsChanged(&'a SingleHyp)    
}
