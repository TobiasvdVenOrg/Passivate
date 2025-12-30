#![feature(trait_alias)]

pub mod bridge;
pub mod bridge_hyp;
pub mod hyp_report;
pub mod hyp_run_bridge;
pub mod hyp_run_trigger;
pub mod hyp_session_bridge;
pub mod hyp_state;
pub mod output_report;

use std::fmt::Debug;

pub trait BridgeType = Clone + Debug + Eq + PartialEq;
