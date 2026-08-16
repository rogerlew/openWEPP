#![allow(clippy::missing_errors_doc)]
//! Digest-bound coupled C3 woody vegetation state machine.

pub mod carbon_nitrogen;
pub mod carbon_phase;
pub mod column;
pub mod config;
pub mod diagnostics;
pub mod energy;
pub mod energy_proposal;
pub mod error;
pub mod hydraulics;
pub mod interception;
pub mod ledger;
pub mod migration;
pub mod model;
pub mod nitrogen_protocol;
pub mod numerics;
pub mod occupancy_solver;
pub mod occupancy_state;
pub(crate) mod persistent_phase;
pub mod photosynthesis;
pub mod radiation;
pub mod transaction;
pub mod v8_state;
pub(crate) mod vegetation_candidate;
pub(crate) mod vegetation_ledger;
pub mod water_phase;

pub use config::*;
pub use error::*;
pub use model::*;
pub use transaction::*;
pub use v8_state::*;
pub use water_phase::*;
