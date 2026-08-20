//! Production implementation of `SC-COUPLEDTIME-001`.
//!
//! This crate owns chronology and atomic transaction mechanics only. Numerical
//! controller policy and constitutive equations remain adopter authority.
#![allow(clippy::missing_errors_doc)]

mod clock;
mod constraint;
mod error;
mod event;
mod identity;
mod restart;
mod support;
mod transaction;
mod wire;

pub use clock::*;
pub use constraint::*;
pub use error::*;
pub use event::*;
pub use identity::*;
pub use restart::*;
pub use support::*;
pub use transaction::*;

/// Canonical authority identity.
pub const AUTHORITY_ID: &str = "OPENWEPP_COUPLED_TIME_SUPPORT_V1";
