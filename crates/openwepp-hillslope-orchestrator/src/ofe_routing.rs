//! MOFEFID Lane D — OFE-by-OFE overland-flow routing (SC-OFEROUTE-001,
//! ADR-0033 ratified). Opt-in, shadow-first subsystem implementing the
//! space/time-variant flow resistance kernels, the D4 single-OFE
//! TVD-MacCormack kinematic-wave solver, and the D5 OFE-by-OFE cascade. No
//! phase-span wiring exists yet.

pub mod cascade;
#[cfg(test)]
mod d10b_reconciliation_tests;
pub mod dval;
pub mod friction;
pub mod infiltration;
pub mod iwagaki_oracle;
pub mod kinematic_wave;
pub mod profile;
pub mod seam;
