#![allow(clippy::missing_errors_doc)]
//! Digest-bound coupled C3 woody vegetation state machine.

pub mod carbon_nitrogen;
pub mod config;
pub mod energy;
pub mod error;
pub mod hydraulics;
pub mod interception;
pub mod ledger;
pub mod migration;
pub mod model;
pub mod numerics;
pub mod occupancy_state;
pub mod photosynthesis;
pub mod radiation;
#[cfg(test)]
mod topology_tests;
pub mod transaction;

pub use config::*;
pub use error::*;
pub use model::*;
pub use transaction::*;
