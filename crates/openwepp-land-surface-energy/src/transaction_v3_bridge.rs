//! Private bridge from the V3 accepted solve into the established physical
//! covered-column envelope. This module assembles operands only; it never
//! invokes a nonlinear solve.

use crate::{
    AcceptedV3CoveredSolve, V3CompleteCoveredTileCandidate, V3CoveredPotentialPhase,
    V3CoveredTileEnergyOperandSet,
};
use std::collections::BTreeMap;

use super::{
    CoveredColumnCandidate, CoveredColumnInputs, CoveredPotentialPhase,
    CoveredVegetationOperandPass, LandSurfaceEnergyError, PotentialCoveredOccupancyCarbonOperands,
    PotentialCoveredVegetationOperands, RootRuntimeIdentity, RuntimeTileIdentity,
    SealedCoveredVegetationOperands, SoilThermalFinalizationBeginning, TileState,
    VEGETATION_MODEL_DEFINITION_SHA256, VEGETATION_MODEL_VERSION, WaterProtocol,
    accepted_covered_vegetation_operands, build_covered_soil_candidate, canonical_digest,
    covered_occupancy_energy_operands, rollback_hashes, validate_sealed_gas_branches,
};

fn physical_candidate(accepted: &AcceptedV3CoveredSolve) -> Box<CoveredColumnCandidate> {
    let evaluation = accepted.evaluation.predecessor.clone();
    let root_water = evaluation
        .occupancies
        .iter()
        .flat_map(|occupancy| occupancy.source_water.clone())
        .collect();
    Box::new(CoveredColumnCandidate {
        solution: accepted.solution.clone(),
        surface_enthalpy_j_m2_tile: evaluation.ending_surface_enthalpy_j_m2_tile,
        soil_temperature_k: evaluation.soil_temperature_k.clone(),
        root_water,
        ground_water: evaluation.ground_water,
        iterations: accepted.iterations,
        backtracking_count: accepted.backtracking_count,
        // The V3 solver scales coordinates so this scalar is a conservative
        // per-family bound. It is in-memory solve metadata, never a persisted
        // V1 numerical-diagnostics claim.
        step_norms: crate::CoveredStepNorms {
            hydraulic_mm: accepted.step_norm * 10.0,
            beta: accepted.step_norm * 0.01,
            temperature_k: accepted.step_norm,
            humidity_kg_kg: accepted.step_norm,
            ci_pa: 0.0,
        },
        evaluation,
    })
}

pub(crate) fn build_v3_potential_vegetation_operands(
    identity: &RuntimeTileIdentity,
    beginning: &CoveredColumnInputs,
    accepted: &AcceptedV3CoveredSolve,
    root_identities: &BTreeMap<(String, String), RootRuntimeIdentity>,
) -> Result<PotentialCoveredVegetationOperands, LandSurfaceEnergyError> {
    let vegetation_owner_id = root_identities
        .values()
        .next()
        .ok_or(LandSurfaceEnergyError::water_cardinality(
            "missing V3 potential vegetation owner",
        ))?
        .requesting_owner_id
        .clone();
    let mut occupancies = Vec::with_capacity(beginning.occupancies.len());
    for (input, evaluation) in beginning
        .occupancies
        .iter()
        .zip(&accepted.evaluation.predecessor.occupancies)
    {
        let runtime = root_identities
            .iter()
            .find_map(|((solver_occupancy, _), runtime)| {
                (solver_occupancy == &input.occupancy_id).then_some(runtime)
            })
            .ok_or(LandSurfaceEnergyError::water_identity(
                "missing V3 potential vegetation occupancy identity",
            ))?;
        occupancies.push(PotentialCoveredOccupancyCarbonOperands {
            occupancy_id: runtime.occupancy_id.clone(),
            sun_leaf_area_m2_m2_tile_ground: input.sun.leaf_area_m2_m2_tile,
            shade_leaf_area_m2_m2_tile_ground: input.shade.leaf_area_m2_m2_tile,
            sun_gross_assimilation_umol_co2_m2_leaf_s: evaluation
                .gross_assimilation_umol_co2_m2_leaf_s[0],
            shade_gross_assimilation_umol_co2_m2_leaf_s: evaluation
                .gross_assimilation_umol_co2_m2_leaf_s[1],
            sun_net_assimilation_umol_co2_m2_leaf_s: evaluation.net_assimilation_umol_co2_m2_leaf_s
                [0],
            shade_net_assimilation_umol_co2_m2_leaf_s: evaluation
                .net_assimilation_umol_co2_m2_leaf_s[1],
            sun_dark_respiration_umol_co2_m2_leaf_s: evaluation.dark_respiration_umol_co2_m2_leaf_s
                [0],
            shade_dark_respiration_umol_co2_m2_leaf_s: evaluation
                .dark_respiration_umol_co2_m2_leaf_s[1],
            liquid: evaluation.liquid,
        });
    }
    let detail = &accepted.evaluation.predecessor;
    let mut operands = PotentialCoveredVegetationOperands {
        pass: CoveredVegetationOperandPass::Potential,
        transaction_id: identity.transaction_id,
        vegetation_model_version: VEGETATION_MODEL_VERSION,
        vegetation_model_definition_sha256: VEGETATION_MODEL_DEFINITION_SHA256,
        lse_configuration_sha256: identity.configuration_sha256.clone(),
        beginning_lse_state_sha256: identity.beginning_lse_state_sha256.clone(),
        vegetation_owner_id,
        ofe_id: identity.ofe_id.clone(),
        tile_id: identity.tile_id.clone(),
        tile_fraction: identity.tile_fraction,
        interval_s: identity.interval_s,
        canopy_air_temperature_k: detail.canopy_air_temperature_k,
        canopy_air_specific_humidity_kg_kg: detail.canopy_air_specific_humidity_kg_kg,
        top_rain_kg_m2_tile_ground: beginning.top_rain_kg_m2_tile,
        ground_canopy_release_kg_m2_tile_ground: detail.ground_canopy_release_kg_m2_tile,
        ground_stemflow_kg_m2_tile_ground: detail.ground_stemflow_kg_m2_tile,
        occupancies,
        payload_sha256: identity.beginning_lse_state_sha256.clone(),
        seal: SealedCoveredVegetationOperands::Potential,
    };
    operands.payload_sha256 = canonical_digest(&operands)?;
    operands.validate()?;
    Ok(operands)
}

pub(crate) fn build_complete_v3_final_candidate(
    phase: &V3CoveredPotentialPhase,
    accepted: &AcceptedV3CoveredSolve,
    water_protocol: &WaterProtocol,
    soil: SoilThermalFinalizationBeginning<'_>,
) -> Result<V3CompleteCoveredTileCandidate, LandSurfaceEnergyError> {
    let potential_physical = physical_candidate(phase.accepted());
    let final_physical = physical_candidate(accepted);
    let gas_branches = phase
        .accepted()
        .evaluation
        .predecessor
        .occupancies
        .iter()
        .map(|occupancy| occupancy.gas_branches)
        .collect();
    let synthetic = CoveredPotentialPhase {
        identity: phase.identity().clone(),
        beginning: phase.beginning().clone(),
        accepted: potential_physical,
        request_batch: phase.request_batch().clone(),
        potential_vegetation_operands: phase.potential_vegetation_operands.clone(),
        root_identities: phase.root_identities().clone(),
        gas_branches,
    };
    let final_gas_branches: Vec<_> = final_physical
        .evaluation
        .occupancies
        .iter()
        .map(|occupancy| occupancy.gas_branches)
        .collect();
    validate_sealed_gas_branches(&synthetic.gas_branches, &final_gas_branches)?;
    let vegetation_operands =
        accepted_covered_vegetation_operands(&synthetic, &final_physical, water_protocol)?;
    let evaluation = &final_physical.evaluation;
    let occupancies = covered_occupancy_energy_operands(&synthetic, evaluation)?;
    let ground_partition = crate::partition_ground_shortwave(
        synthetic.beginning.ground.terminal_shortwave_w_m2_tile,
        synthetic.beginning.ground.surface_vis_albedo,
        synthetic.beginning.ground.surface_nir_albedo,
    )?;
    let longwave = &evaluation.whole_column_longwave;
    let energy_operands = V3CoveredTileEnergyOperandSet {
        authority: synthetic.beginning.authority,
        occupancies,
        canopy_air: crate::CoveredCanopyAirEnergyOperands {
            rho_air_kg_m3: synthetic.beginning.pressure_pa
                / (287.05 * evaluation.canopy_air_temperature_k),
            cp_air_j_kg_k: 1_004.64,
            canopy_air_temperature_k: evaluation.canopy_air_temperature_k,
            canopy_air_specific_humidity_kg_kg: evaluation.canopy_air_specific_humidity_kg_kg,
            ground_sensible_to_canopy_air_w_m2_tile: evaluation.ground_sensible_to_canopy_air_w_m2,
            ground_vapor_to_canopy_air_kg_m2_tile_s: evaluation
                .lower_boundary_vapor_to_canopy_air_kg_m2_s,
            canopy_sensible_w_m2_tile: evaluation.canopy_sensible_w_m2,
            canopy_vapor_kg_m2_tile_s: evaluation.canopy_vapor_kg_m2_s,
            sensible_to_reference_air_w_m2_tile: evaluation.sensible_to_reference_air_w_m2,
            vapor_to_reference_air_kg_m2_tile_s: evaluation.vapor_to_reference_air_kg_m2_s,
            shared_heat_residual_w_m2_tile: evaluation.shared_heat_residual_w_m2,
            shared_heat_tolerance_w_m2_tile: evaluation.shared_heat_tolerance_w_m2,
            shared_vapor_residual_kg_m2_tile_s: evaluation.shared_vapor_residual_kg_m2_s,
            shared_vapor_tolerance_kg_m2_tile_s: evaluation.shared_vapor_tolerance_kg_m2_s,
        },
        shortwave: crate::CoveredColumnShortwaveOperands {
            incident_w_m2_tile: synthetic.beginning.shortwave.incident_w_m2_tile,
            top_reflected_w_m2_tile: synthetic.beginning.shortwave.top_reflected_w_m2_tile,
            ground_absorbed_by_incident_w_m2_tile: synthetic
                .beginning
                .shortwave
                .ground_absorbed_by_incident_w_m2_tile,
            ground_terminal_w_m2_tile: synthetic.beginning.ground.terminal_shortwave_w_m2_tile,
            ground_absorbed_w_m2_tile: ground_partition.absorbed,
            ground_reflected_w_m2_tile: ground_partition.reflected,
            occupancies: synthetic.beginning.shortwave.occupancies.clone(),
        },
        longwave: crate::CoveredColumnLongwaveOperands {
            atmospheric_downward_w_m2_tile: synthetic.beginning.atmospheric_downward_longwave_w_m2,
            transmissivities: longwave.transmissivities.clone(),
            downward_boundaries_w_m2_tile: longwave.downward_boundaries_w_m2.clone(),
            upward_boundaries_w_m2_tile: longwave.upward_boundaries_w_m2.clone(),
            top_upward_w_m2_tile: longwave.top_upward_w_m2,
            ground_net_w_m2_tile: longwave.ground_net_w_m2,
            occupancy_component_net_w_m2_tile: synthetic
                .beginning
                .occupancies
                .iter()
                .zip(&longwave.component_net_w_m2)
                .map(|(input, values)| (input.occupancy_id.clone(), *values))
                .collect(),
        },
        litter_surface: accepted.evaluation.surface_energy,
    };
    let soil_thermal = build_covered_soil_candidate(&synthetic, &final_physical, soil)?;
    Ok(V3CompleteCoveredTileCandidate {
        transaction_id: phase.identity().transaction_id,
        identity: phase.identity().clone(),
        ending_tile_state_pre_ingress: TileState {
            ofe_id: phase.identity().ofe_id.clone(),
            tile_id: phase.identity().tile_id.clone(),
            surface_enthalpy_j_m2_tile_ground: final_physical.surface_enthalpy_j_m2_tile,
            surface_temperature_warm_start_k: final_physical.evaluation.ground_temperature_k,
        },
        final_solver_candidate: final_physical,
        water_protocol: water_protocol.clone(),
        soil_thermal,
        energy_operands,
        rollback_hashes: rollback_hashes(phase.identity()),
        vegetation_operands,
    })
}
