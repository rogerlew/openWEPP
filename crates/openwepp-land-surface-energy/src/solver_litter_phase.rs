//! Phase-free V3 vapor publication seam.
//!
//! The predecessor solver has already converged before this module is called.
//! No phase mass, fusion term, or phase-adjusted temperature enters Newton.

#![allow(clippy::missing_errors_doc)]

use crate::{
    BeginningLitterPhaseState, LandSurfaceEnergyError, LitterPhaseConfiguration,
    LitterVaporEnvironment, RawLitterVapor, evaluate_raw_litter_vapor,
};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PhaseFreeLitterSolveOutput {
    pub accepted_surface_temperature_k: f64,
    pub raw_vapor: RawLitterVapor,
    pub nonlinear_phase_evaluation_count: u8,
}

/// Publish V3 phase-specific vapor from an already accepted V2 solve. The
/// zero counter is a structural receipt that phase never entered the solve.
pub fn publish_phase_free_litter_vapor(
    configuration: LitterPhaseConfiguration,
    beginning: BeginningLitterPhaseState,
    environment: LitterVaporEnvironment,
) -> Result<PhaseFreeLitterSolveOutput, LandSurfaceEnergyError> {
    let raw_vapor = evaluate_raw_litter_vapor(configuration, beginning, environment)?;
    Ok(PhaseFreeLitterSolveOutput {
        accepted_surface_temperature_k: environment.accepted_phase_free_temperature_k,
        raw_vapor,
        nonlinear_phase_evaluation_count: 0,
    })
}
