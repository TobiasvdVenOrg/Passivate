use passivate_hyp_model::{hyp_run_events::HypRunChange, single_hyp::SingleHyp};


pub enum PassivateStateChange<'a>
{
    HypDetailsChanged(&'a SingleHyp)    
}
