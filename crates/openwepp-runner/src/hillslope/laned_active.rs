//! Lane D ACTIVE production owner selector (`SC-OFEROUTE-001` rev 27,
//! `INV-OFEROUTE-012` activation): opt-in via `OPENWEPP_LANED_ACTIVE=1` on
//! the production publication-stream path. The runner builds the per-lane
//! active configuration from the SAME rev-20/21 authority sources as the
//! diagnostic shadow (native management `routing_coefficients`, Wave-1
//! operand-seed geometry, typed-management `canhgt`) and attaches it to the
//! direct run frame; the orchestrator executor then owns the two-phase
//! active day loop, the DC01-surface-disable, the D13 erosion producer
//! flip, and the day-closure hard-fails. Mutually exclusive with
//! `OPENWEPP_LANED_SHADOW=1` (the shadow's published-row reconstruction
//! basis is DC01-shaped and is not defined over an active run).

/// Env opt-in: `OPENWEPP_LANED_ACTIVE=1`.
#[must_use]
pub(crate) fn env_enabled() -> bool {
    std::env::var("OPENWEPP_LANED_ACTIVE").is_ok_and(|value| value == "1")
}

/// T3-I2 experimental opt-in: `OPENWEPP_LANED_ACTIVE_IMPLICIT=1` (composes
/// with the active selector; rev-28 hybrid stepping).
#[must_use]
pub(crate) fn env_hybrid_implicit_enabled() -> bool {
    std::env::var("OPENWEPP_LANED_ACTIVE_IMPLICIT").is_ok_and(|value| value == "1")
}
