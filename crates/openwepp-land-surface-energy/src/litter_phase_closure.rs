//! Independent primitive-operand closure for the V3 litter transaction.

#![allow(clippy::missing_errors_doc)]

use crate::{
    BeginningLitterPhaseState, EndingLitterPhaseState, LandSurfaceEnergyError, LitterPhaseClosure,
    LitterPhaseConfiguration, LitterPhaseTransfer, LitterVaporReceipt, PostVaporLitterState,
    physics::{
        LITTER_FUSION_ENTHALPY_J_KG, LITTER_ICE_HEAT_CAPACITY_J_KG_K, REFERENCE_TEMPERATURE_K,
        WATER_HEAT_CAPACITY_J_KG_K,
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

/// Reconstruct all mass, vapor-energy, fusion-energy, enthalpy-coordinate and
/// ending-capacity identities without consuming a producer residual.
pub fn reconstruct_litter_phase_closure(
    configuration: LitterPhaseConfiguration,
    beginning: BeginningLitterPhaseState,
    vapor: LitterVaporReceipt,
    post_vapor: PostVaporLitterState,
    transfer: LitterPhaseTransfer,
    ending: EndingLitterPhaseState,
) -> Result<LitterPhaseClosure, LandSurfaceEnergyError> {
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
    ] {
        require_close(residual, scale, detail)?;
    }
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
    })
}
