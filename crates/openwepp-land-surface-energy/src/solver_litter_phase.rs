//! Phase-free V3 vapor publication and current-trial residual operands.

#![allow(clippy::missing_errors_doc)]

use crate::{
    BeginningLitterPhaseState, CoveredColumnEvaluation, CoveredColumnInputs, CoveredFrozenBranches,
    CoveredWaterCaps, FinalizedLitterVapor, LandSurfaceEnergyError, LitterPhaseConfiguration,
    LitterVaporEnvironment, LitterVaporReceipt, NormalizedSolveOutcome, PostVaporLitterState,
    RawLitterVapor, evaluate_raw_litter_vapor, solve_normalized_system,
};
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PhaseFreeLitterSolveOutput {
    pub accepted_surface_temperature_k: f64,
    pub raw_vapor: RawLitterVapor,
    pub nonlinear_phase_evaluation_count: u8,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct V3LitterResidualContext {
    pub configuration: LitterPhaseConfiguration,
    pub beginning: BeginningLitterPhaseState,
    /// Absent for the potential/request solve; present for the fixed-
    /// authorization final solve. Negative raw components remain exact.
    pub finalized_vapor: Option<FinalizedLitterVapor>,
}

#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct V3PhaseFreeSurfaceEnergyLedger {
    pub beginning_sensible_energy_j_m2: f64,
    pub ending_sensible_energy_j_m2: f64,
    pub absorbed_shortwave_w_m2: f64,
    pub net_longwave_w_m2: f64,
    pub sensible_to_canopy_air_w_m2: f64,
    pub liquid_vapor_energy_w_m2: f64,
    pub ice_vapor_energy_w_m2: f64,
    pub ground_heat_w_m2: f64,
    pub storage_w_m2: f64,
    pub reconstructed_energy_residual_w_m2: f64,
}

#[derive(Clone, Debug, PartialEq)]
pub struct V3PhaseFreeCoveredEvaluation {
    pub predecessor: CoveredColumnEvaluation,
    pub vapor: LitterVaporReceipt,
    pub post_vapor: PostVaporLitterState,
    pub surface_energy: V3PhaseFreeSurfaceEnergyLedger,
}

/// Publish V3 phase-specific vapor from a current trial. The zero phase
/// counter records that freeze/melt never entered Newton.
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

/// Evaluate the complete covered-column residual with phase-specific V3
/// vapor and heat capacity at the current trial. Phase transfer remains
/// post-solve and never enters this residual.
pub fn evaluate_v3_phase_free_covered_column(
    column: &CoveredColumnInputs,
    trial: &[f64],
    caps: Option<&CoveredWaterCaps>,
    frozen: Option<&CoveredFrozenBranches>,
    context: V3LitterResidualContext,
) -> Result<V3PhaseFreeCoveredEvaluation, LandSurfaceEnergyError> {
    crate::solver::evaluate_covered_column_v3(column, trial, caps, frozen, context)
}

fn freeze_v3_covered_branches(evaluation: &V3PhaseFreeCoveredEvaluation) -> CoveredFrozenBranches {
    let detail = &evaluation.predecessor;
    let mut frozen = CoveredFrozenBranches {
        ground: Some(detail.ground_water.branch),
        ..Default::default()
    };
    for occupancy in &detail.occupancies {
        if let Some(identity) = occupancy
            .source_water
            .first()
            .map(|source| source.occupancy_id.clone())
        {
            frozen.wet.insert(identity, occupancy.wet_branch);
        }
        for source in &occupancy.source_water {
            frozen.root.insert(
                (source.occupancy_id.clone(), source.layer_id.clone()),
                source.branch,
            );
        }
    }
    frozen
}

fn v3_step_coordinate_scales(occupancy_count: usize, soil_node_count: usize) -> Vec<f64> {
    (0..occupancy_count)
        .flat_map(|_| [10.0, 10.0, 10.0, 10.0, 0.01, 0.01, 1.0, 1.0, 1.0, 1.0])
        .chain([1.0, 1.0e-4, 1.0])
        .chain(std::iter::repeat_n(1.0, soil_node_count))
        .collect()
}

fn scaled_trial(values: &[f64], coordinate_scales: &[f64]) -> Vec<f64> {
    values
        .iter()
        .zip(coordinate_scales)
        .map(|(value, scale)| value / scale)
        .collect()
}

fn physical_trial(values: &[f64], coordinate_scales: &[f64]) -> Vec<f64> {
    values
        .iter()
        .zip(coordinate_scales)
        .map(|(value, scale)| value * scale)
        .collect()
}

/// Solve the covered V3 phase-free system with phase-specific vapor present
/// in every residual and finite-difference evaluation. Coordinate scaling
/// preserves the governed covered-column step thresholds: `1e-7 mm`,
/// `1e-10` beta, `1e-8 K`, and `1e-12 kg kg-1` humidity.
pub fn solve_v3_phase_free_covered_column(
    column: &CoveredColumnInputs,
    caps: Option<&CoveredWaterCaps>,
    initial_trial: &[f64],
    context: V3LitterResidualContext,
) -> Result<NormalizedSolveOutcome<V3PhaseFreeCoveredEvaluation>, LandSurfaceEnergyError> {
    let coordinate_scales =
        v3_step_coordinate_scales(column.occupancies.len(), column.ground.soil_nodes.len());
    if initial_trial.len() != coordinate_scales.len() {
        return Err(LandSurfaceEnergyError::topology_domain(
            "V3 phase-free covered solver shape",
        ));
    }
    let scaled_initial = scaled_trial(initial_trial, &coordinate_scales);
    let physical_units: Vec<f64> = (0..column.occupancies.len())
        .flat_map(|_| [1000.0, 1000.0, 1000.0, 1000.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0])
        .chain([1.0, 0.001, 1.0])
        .chain(std::iter::repeat_n(1.0, column.ground.soil_nodes.len()))
        .collect();
    let scaled_units: Vec<f64> = physical_units
        .iter()
        .zip(&coordinate_scales)
        .map(|(unit, scale)| unit / scale)
        .collect();
    let result = solve_normalized_system(
        |scaled: &[f64], frozen: Option<&CoveredFrozenBranches>| {
            let physical = physical_trial(scaled, &coordinate_scales);
            let evaluation =
                evaluate_v3_phase_free_covered_column(column, &physical, caps, frozen, context)?;
            Ok((
                evaluation.predecessor.normalized_residuals.clone(),
                evaluation,
            ))
        },
        scaled_initial,
        &scaled_units,
        |scaled: &[f64]| {
            let physical = physical_trial(scaled, &coordinate_scales);
            crate::solver::covered_trial_is_valid(
                &physical,
                column.occupancies.len(),
                crate::solver::covered_ground_uses_liquid_vapor_phase_domain(column),
            )
        },
        freeze_v3_covered_branches,
    )?;
    Ok(match result {
        NormalizedSolveOutcome::Accepted {
            solution,
            detail,
            iterations,
            residual_norm_history,
            backtracking_count,
            step_norm,
            pivot_magnitude,
            matrix_norm,
        } => NormalizedSolveOutcome::Accepted {
            solution: physical_trial(&solution, &coordinate_scales),
            detail,
            iterations,
            residual_norm_history,
            backtracking_count,
            step_norm,
            pivot_magnitude,
            matrix_norm,
        },
        NormalizedSolveOutcome::Rejected(failure) => NormalizedSolveOutcome::Rejected(failure),
    })
}
