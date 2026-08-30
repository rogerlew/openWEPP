//! Independent primitive-operand closure for the V3 litter transaction.

#![allow(clippy::missing_errors_doc)]

use crate::{
    BeginningLitterPhaseState, EndingLitterPhaseState, LandSurfaceEnergyError, LitterPhaseClosure,
    LitterPhaseConfiguration, LitterPhaseTransfer, LitterVaporReceipt, PostVaporLitterState,
    V3PhaseFreeSurfaceEnergyLedger,
    physics::{
        LITTER_FUSION_ENTHALPY_J_KG, LITTER_ICE_HEAT_CAPACITY_J_KG_K, REFERENCE_TEMPERATURE_K,
        WATER_HEAT_CAPACITY_J_KG_K, energy_tolerance,
    },
};

fn closure_tolerance(scale: f64) -> f64 {
    1.0e-7 + 64.0 * f64::EPSILON * scale.abs().max(1.0)
}

fn require_close(
    residual: f64,
    scale: f64,
    detail: &'static str,
) -> Result<(), LandSurfaceEnergyError> {
    if !residual.is_finite() || residual.abs() > closure_tolerance(scale) {
        Err(LandSurfaceEnergyError::FrozenLitterPhaseClosure(detail))
    } else {
        Ok(())
    }
}

fn require_energy_close(
    residual_w_m2: f64,
    scale_w_m2: f64,
    detail: &'static str,
) -> Result<(), LandSurfaceEnergyError> {
    if !residual_w_m2.is_finite() || residual_w_m2.abs() > energy_tolerance(scale_w_m2.abs()) {
        Err(LandSurfaceEnergyError::FrozenLitterPhaseClosure(detail))
    } else {
        Ok(())
    }
}

/// Reconstruct all mass, vapor-energy, fusion-energy, enthalpy-coordinate and
/// ending-capacity identities without consuming a producer residual.
#[allow(clippy::too_many_arguments, clippy::too_many_lines)]
pub fn reconstruct_litter_phase_closure(
    configuration: LitterPhaseConfiguration,
    beginning: BeginningLitterPhaseState,
    vapor: LitterVaporReceipt,
    post_vapor: PostVaporLitterState,
    phase_free_surface_energy: V3PhaseFreeSurfaceEnergyLedger,
    interval_s: f64,
    transfer: LitterPhaseTransfer,
    ending: EndingLitterPhaseState,
) -> Result<LitterPhaseClosure, LandSurfaceEnergyError> {
    if !interval_s.is_finite() || interval_s <= 0.0 {
        return Err(LandSurfaceEnergyError::FrozenLitterPhaseClosure(
            "phase-free support duration",
        ));
    }
    let expected_post_liquid = beginning.liquid_kg_m2_tile - vapor.liquid_signed_mass_kg_m2;
    let expected_post_ice = beginning.ice_kg_m2_tile - vapor.ice_signed_mass_kg_m2;
    let liquid_residual = ending.liquid_kg_m2_tile
        - (expected_post_liquid - transfer.freeze_kg_m2 + transfer.melt_kg_m2);
    let ice_residual =
        ending.ice_kg_m2_tile - (expected_post_ice + transfer.freeze_kg_m2 - transfer.melt_kg_m2);
    let total_residual = (ending.liquid_kg_m2_tile + ending.ice_kg_m2_tile)
        - (expected_post_liquid + expected_post_ice);
    let expected_fusion =
        LITTER_FUSION_ENTHALPY_J_KG * (transfer.freeze_kg_m2 - transfer.melt_kg_m2);
    let fusion_residual = transfer.fusion_energy_j_m2 - expected_fusion;
    let post_enthalpy = post_vapor.sensible_energy_j_m2_tile
        - LITTER_FUSION_ENTHALPY_J_KG * post_vapor.ice_kg_m2_tile;
    let ending_enthalpy =
        ending.sensible_energy_j_m2_tile - LITTER_FUSION_ENTHALPY_J_KG * ending.ice_kg_m2_tile;
    let phase_enthalpy_residual = ending_enthalpy - post_enthalpy;
    let expected_capacity = configuration.dry_heat_capacity_j_m2_k
        + ending.liquid_kg_m2_tile * WATER_HEAT_CAPACITY_J_KG_K
        + ending.ice_kg_m2_tile * LITTER_ICE_HEAT_CAPACITY_J_KG_K;
    let expected_temperature =
        REFERENCE_TEMPERATURE_K + ending.sensible_energy_j_m2_tile / expected_capacity;
    let temperature_residual = ending.temperature_k - expected_temperature;
    let expected_liquid_vapor_energy =
        vapor.liquid_signed_mass_kg_m2 * vapor.liquid_specific_enthalpy_j_kg;
    let expected_ice_vapor_energy = vapor.ice_signed_mass_kg_m2 * vapor.ice_specific_enthalpy_j_kg;
    let liquid_vapor_residual = vapor.liquid_signed_energy_j_m2 - expected_liquid_vapor_energy;
    let ice_vapor_residual = vapor.ice_signed_energy_j_m2 - expected_ice_vapor_energy;
    let expected_storage =
        (post_vapor.sensible_energy_j_m2_tile - beginning.sensible_energy_j_m2_tile) / interval_s;
    let phase_free_storage_residual = phase_free_surface_energy.storage_w_m2 - expected_storage;
    let reconstructed_surface_energy = phase_free_surface_energy.absorbed_shortwave_w_m2
        + phase_free_surface_energy.net_longwave_w_m2
        - phase_free_surface_energy.sensible_to_canopy_air_w_m2
        - phase_free_surface_energy.liquid_vapor_energy_w_m2
        - phase_free_surface_energy.ice_vapor_energy_w_m2
        - phase_free_surface_energy.ground_heat_w_m2
        - expected_storage;
    let producer_residual_delta =
        phase_free_surface_energy.reconstructed_energy_residual_w_m2 - reconstructed_surface_energy;
    let beginning_energy_join = phase_free_surface_energy.beginning_sensible_energy_j_m2
        - beginning.sensible_energy_j_m2_tile;
    let ending_energy_join = phase_free_surface_energy.ending_sensible_energy_j_m2
        - post_vapor.sensible_energy_j_m2_tile;
    let liquid_vapor_flux_join = phase_free_surface_energy.liquid_vapor_energy_w_m2 * interval_s
        - vapor.liquid_signed_energy_j_m2;
    let ice_vapor_flux_join =
        phase_free_surface_energy.ice_vapor_energy_w_m2 * interval_s - vapor.ice_signed_energy_j_m2;
    let surface_scale = phase_free_surface_energy.absorbed_shortwave_w_m2.abs()
        + phase_free_surface_energy.net_longwave_w_m2.abs()
        + phase_free_surface_energy.sensible_to_canopy_air_w_m2.abs()
        + phase_free_surface_energy.liquid_vapor_energy_w_m2.abs()
        + phase_free_surface_energy.ice_vapor_energy_w_m2.abs()
        + phase_free_surface_energy.ground_heat_w_m2.abs()
        + expected_storage.abs();

    for (residual, scale, detail) in [
        (
            liquid_residual,
            beginning.liquid_kg_m2_tile + ending.liquid_kg_m2_tile,
            "liquid equal-mass transfer",
        ),
        (
            ice_residual,
            beginning.ice_kg_m2_tile + ending.ice_kg_m2_tile,
            "ice equal-mass transfer",
        ),
        (
            total_residual,
            beginning.liquid_kg_m2_tile
                + beginning.ice_kg_m2_tile
                + ending.liquid_kg_m2_tile
                + ending.ice_kg_m2_tile,
            "phase total-mass conservation",
        ),
        (
            fusion_residual,
            expected_fusion,
            "fusion-energy sign or magnitude",
        ),
        (
            phase_enthalpy_residual,
            post_enthalpy.abs().max(ending_enthalpy.abs()),
            "U-L_f*W_i phase enthalpy",
        ),
        (
            temperature_residual,
            expected_temperature,
            "ending-capacity temperature",
        ),
        (
            liquid_vapor_residual,
            expected_liquid_vapor_energy,
            "liquid vapor sensible-plus-latent energy",
        ),
        (
            ice_vapor_residual,
            expected_ice_vapor_energy,
            "ice vapor sensible-plus-latent energy",
        ),
        (
            beginning_energy_join,
            beginning.sensible_energy_j_m2_tile,
            "phase-free beginning-energy join",
        ),
        (
            ending_energy_join,
            post_vapor.sensible_energy_j_m2_tile,
            "phase-free post-vapor energy join",
        ),
        (
            liquid_vapor_flux_join,
            vapor.liquid_signed_energy_j_m2,
            "phase-free liquid vapor-energy join",
        ),
        (
            ice_vapor_flux_join,
            vapor.ice_signed_energy_j_m2,
            "phase-free ice vapor-energy join",
        ),
    ] {
        require_close(residual, scale, detail)?;
    }
    require_energy_close(
        phase_free_storage_residual,
        expected_storage,
        "phase-free beginning-to-post-vapor storage",
    )?;
    require_energy_close(
        reconstructed_surface_energy,
        surface_scale,
        "phase-free complete surface-energy ledger",
    )?;
    require_energy_close(
        producer_residual_delta,
        surface_scale,
        "phase-free producer/reconstructed energy residual",
    )?;
    if ending.heat_capacity_j_m2_k.to_bits() != expected_capacity.to_bits() {
        return Err(LandSurfaceEnergyError::FrozenLitterPhaseClosure(
            "ending heat-capacity operand",
        ));
    }
    Ok(LitterPhaseClosure {
        liquid_mass_residual_kg_m2: liquid_residual,
        ice_mass_residual_kg_m2: ice_residual,
        total_phase_mass_residual_kg_m2: total_residual,
        fusion_energy_residual_j_m2: fusion_residual,
        phase_enthalpy_residual_j_m2: phase_enthalpy_residual,
        ending_temperature_residual_k: temperature_residual,
        liquid_vapor_energy_residual_j_m2: liquid_vapor_residual,
        ice_vapor_energy_residual_j_m2: ice_vapor_residual,
        phase_free_storage_residual_w_m2: phase_free_storage_residual,
        phase_free_surface_energy_residual_w_m2: reconstructed_surface_energy,
        phase_free_producer_residual_delta_w_m2: producer_residual_delta,
    })
}
