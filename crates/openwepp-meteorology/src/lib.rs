//! Meteorological primitives for candidate openWEPP snow/frost physics.
//!
//! This crate is intentionally production-free in SNOWDENSITY-10.3.5a. It
//! provides reusable typed psychrometric helpers and Harder-Pomeroy
//! hydrometeor-temperature phase fractions without wiring them into WEPP `RST`
//! precipitation partitioning.

#![deny(unsafe_code)]
#![allow(clippy::module_name_repetitions)]

pub mod error;
pub mod phase;
pub mod psychrometrics;
pub mod snow_free_forcing;
pub mod surface_energy;

pub use error::MeteorologyError;
