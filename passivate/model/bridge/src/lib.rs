#![feature(trait_alias)]

#[macro_use]
extern crate enum_display_derive;

pub mod bridge;
pub mod bridge_hyp;
pub mod hyp_report;
pub mod hyp_run_bridge;
pub mod hyp_session_bridge;
pub mod hyp_session_event;
pub mod hyp_state;
pub mod output_report;

use std::fmt::Debug;

pub trait BridgeType = Clone + Debug + Eq + PartialEq + Send + Sync + 'static;
