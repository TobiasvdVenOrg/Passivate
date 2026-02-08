#![feature(trait_alias)]

pub mod bridge;
pub mod bridge_hyp;
pub mod hyp_report;
pub mod hyp_run_bridge;
pub mod hyp_run_request;
pub mod hyp_session_bridge;
pub mod hyp_session_event;
pub mod hyp_state;
pub mod output_report;
pub mod source_change_bridge;
pub mod source_change_event;

use std::fmt::Debug;

pub trait BridgeType = Clone + Debug + PartialEq + Eq + Send + Sync + 'static;
