//! MOFEFID Lane D — OFE-by-OFE overland-flow routing (SC-OFEROUTE-001,
//! ADR-0033 Proposed). Opt-in, shadow-first subsystem implementing the
//! space/time-variant flow resistance and (in later stages) the
//! TVD-MacCormack kinematic-wave routing of Papanicolaou et al. (2018).
//! D3 lands the friction-factor kernels only; no phase-span wiring.

pub mod friction;
