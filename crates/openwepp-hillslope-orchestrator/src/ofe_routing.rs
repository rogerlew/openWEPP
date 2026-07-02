//! MOFEFID Lane D — OFE-by-OFE overland-flow routing (SC-OFEROUTE-001,
//! ADR-0033 ratified). Opt-in, shadow-first subsystem implementing the
//! space/time-variant flow resistance kernels and the D4 single-OFE
//! TVD-MacCormack kinematic-wave solver. No phase-span wiring exists yet.

pub mod cascade;
pub mod friction;
pub mod kinematic_wave;
