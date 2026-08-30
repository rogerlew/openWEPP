//! Pure V3 forest-litter vapor and bounded kinetic phase primitives.

#![allow(clippy::missing_errors_doc)]

use std::f64::consts::PI;

use crate::{
    BeginningLitterPhaseState, EndingLitterPhaseState, FinalizedLitterVapor,
    LandSurfaceEnergyError, LitterPhaseConfiguration, LitterPhaseTransfer, LitterVaporEnvironment,
    LitterVaporReceipt, PostVaporLitterState, RawLitterVapor,
    physics::{
        LITTER_FUSION_ENTHALPY_J_KG, LITTER_ICE_DENSITY_KG_M3, LITTER_ICE_HEAT_CAPACITY_J_KG_K,
        LITTER_ICE_TIMESCALE_S, LITTER_ICE_VOLUMETRIC_CAPACITY, REFERENCE_TEMPERATURE_K,
        WATER_DENSITY_KG_M3, WATER_HEAT_CAPACITY_J_KG_K, saturation_specific_humidity,
        sublimation_enthalpy_j_kg, vaporization_enthalpy_j_kg,
    },
};

fn finite(value: f64, detail: &'static str) -> Result<f64, LandSurfaceEnergyError> {
    if value.is_finite() {
        Ok(value)
    } else {
        Err(LandSurfaceEnergyError::FrozenLitterV3Identity(detail))
    }
}

fn finite_positive(value: f64, detail: &'static str) -> Result<f64, LandSurfaceEnergyError> {
    finite(value, detail)?;
    if value > 0.0 {
        Ok(value)
    } else {
        Err(LandSurfaceEnergyError::FrozenLitterV3Identity(detail))
    }
}

fn finite_nonnegative(value: f64, detail: &'static str) -> Result<f64, LandSurfaceEnergyError> {
    finite(value, detail)?;
    if value >= 0.0 {
        Ok(value)
    } else {
        Err(LandSurfaceEnergyError::FrozenLitterV3Identity(detail))
    }
}

/// Validate the exact selected V3 phase configuration, including the
/// liquid-water-equivalent ice-capacity basis.
pub fn validate_litter_phase_configuration(
    configuration: LitterPhaseConfiguration,
) -> Result<(), LandSurfaceEnergyError> {
    finite_positive(configuration.litter_depth_m, "litter_depth_m")?;
    finite_positive(
        configuration.dry_heat_capacity_j_m2_k,
        "dry_heat_capacity_j_m2_k",
    )?;
    finite_positive(
        configuration.liquid_capacity_kg_m2_tile,
        "liquid_capacity_kg_m2_tile",
    )?;
    finite_positive(
        configuration.ice_capacity_kg_m2_tile,
        "ice_capacity_kg_m2_tile",
    )?;
    let selected_capacity =
        LITTER_ICE_VOLUMETRIC_CAPACITY * WATER_DENSITY_KG_M3 * configuration.litter_depth_m;
    if configuration.ice_capacity_kg_m2_tile.to_bits() != selected_capacity.to_bits() {
        return Err(LandSurfaceEnergyError::FrozenLitterV3Identity(
            "ice capacity must be 0.85*rho_w*litter_depth",
        ));
    }
    Ok(())
}

pub fn validate_beginning_litter_state(
    configuration: LitterPhaseConfiguration,
    state: BeginningLitterPhaseState,
) -> Result<(), LandSurfaceEnergyError> {
    validate_litter_phase_configuration(configuration)?;
    finite_nonnegative(state.liquid_kg_m2_tile, "beginning liquid")?;
    finite_nonnegative(state.ice_kg_m2_tile, "beginning ice")?;
    finite(state.sensible_energy_j_m2_tile, "beginning sensible energy")?;
    finite_positive(state.temperature_k, "beginning temperature")?;
    if !(200.0..=350.0).contains(&state.temperature_k)
        || state.liquid_kg_m2_tile > configuration.liquid_capacity_kg_m2_tile
        || state.ice_kg_m2_tile > configuration.ice_capacity_kg_m2_tile
    {
        return Err(LandSurfaceEnergyError::FrozenLitterV3Identity(
            "beginning phase-state domain",
        ));
    }
    let capacity = configuration.dry_heat_capacity_j_m2_k
        + state.liquid_kg_m2_tile * WATER_HEAT_CAPACITY_J_KG_K
        + state.ice_kg_m2_tile * LITTER_ICE_HEAT_CAPACITY_J_KG_K;
    let reconstructed = capacity * (state.temperature_k - REFERENCE_TEMPERATURE_K);
    let tolerance = 1.0e-7 + 64.0 * f64::EPSILON * reconstructed.abs().max(1.0);
    if (state.sensible_energy_j_m2_tile - reconstructed).abs() > tolerance {
        return Err(LandSurfaceEnergyError::FrozenLitterPhaseClosure(
            "beginning sensible-energy coordinate",
        ));
    }
    Ok(())
}

#[must_use]
pub fn frozen_fraction(liquid_kg_m2: f64, ice_kg_m2: f64) -> f64 {
    let total = liquid_kg_m2 + ice_kg_m2;
    if total == 0.0 { 0.0 } else { ice_kg_m2 / total }
}

fn interception_factor(amount: f64, capacity: f64) -> f64 {
    0.5 * (1.0 - (PI * amount / capacity).cos())
}

/// Evaluate the two phase-free vapor components from immutable beginning
/// availability. Both deliberately use liquid-water saturation humidity.
pub fn evaluate_raw_litter_vapor(
    configuration: LitterPhaseConfiguration,
    beginning: BeginningLitterPhaseState,
    environment: LitterVaporEnvironment,
) -> Result<RawLitterVapor, LandSurfaceEnergyError> {
    validate_beginning_litter_state(configuration, beginning)?;
    for (value, detail) in [
        (
            environment.accepted_phase_free_temperature_k,
            "accepted phase-free temperature",
        ),
        (environment.air_density_kg_m3, "air density"),
        (environment.air_pressure_pa, "air pressure"),
        (
            environment.recipient_specific_humidity_kg_kg,
            "recipient specific humidity",
        ),
        (
            environment.litter_to_canopy_resistance_s_m,
            "litter-to-canopy resistance",
        ),
    ] {
        finite(value, detail)?;
    }
    if !(200.0..=350.0).contains(&environment.accepted_phase_free_temperature_k)
        || environment.air_density_kg_m3 <= 0.0
        || environment.air_pressure_pa <= 0.0
        || !(0.0..=0.1).contains(&environment.recipient_specific_humidity_kg_kg)
        || environment.litter_to_canopy_resistance_s_m <= 0.0
    {
        return Err(LandSurfaceEnergyError::FrozenLitterVapor(
            "phase-free vapor environment domain",
        ));
    }
    let frozen = frozen_fraction(beginning.liquid_kg_m2_tile, beginning.ice_kg_m2_tile);
    let liquid_factor = interception_factor(
        beginning.liquid_kg_m2_tile,
        configuration.liquid_capacity_kg_m2_tile,
    );
    let ice_factor = interception_factor(
        beginning.ice_kg_m2_tile,
        configuration.ice_capacity_kg_m2_tile,
    );
    let saturation = saturation_specific_humidity(
        environment.accepted_phase_free_temperature_k,
        environment.air_pressure_pa,
    )?;
    let common = environment.air_density_kg_m3
        * (saturation - environment.recipient_specific_humidity_kg_kg)
        / environment.litter_to_canopy_resistance_s_m;
    let liquid = (1.0 - frozen) * liquid_factor * common;
    let ice = frozen * ice_factor * common;
    if !liquid.is_finite() || !ice.is_finite() {
        return Err(LandSurfaceEnergyError::FrozenLitterVapor(
            "nonfinite raw phase-specific vapor",
        ));
    }
    Ok(RawLitterVapor {
        environment,
        frozen_fraction: frozen,
        liquid_interception_factor: liquid_factor,
        ice_interception_factor: ice_factor,
        liquid_saturation_specific_humidity_kg_kg: saturation,
        raw_liquid_signed_rate_kg_m2_s: liquid,
        raw_ice_signed_rate_kg_m2_s: ice,
    })
}

fn validate_final_component(
    raw_rate: f64,
    finalized_rate: f64,
    available_mass: f64,
    interval_s: f64,
) -> Result<(), LandSurfaceEnergyError> {
    if !finalized_rate.is_finite() {
        return Err(LandSurfaceEnergyError::FrozenLitterVapor(
            "nonfinite finalized vapor",
        ));
    }
    if raw_rate < 0.0 {
        if finalized_rate.to_bits() != raw_rate.to_bits() {
            return Err(LandSurfaceEnergyError::FrozenLitterVapor(
                "inbound vapor must credit its named phase exactly",
            ));
        }
    } else {
        let maximum = raw_rate.min(available_mass / interval_s);
        if finalized_rate < 0.0 || finalized_rate > maximum {
            return Err(LandSurfaceEnergyError::FrozenLitterVapor(
                "outbound vapor exceeds raw request or named beginning phase",
            ));
        }
    }
    Ok(())
}

/// Seal finalized phase-specific vapor and its distinct sensible-plus-latent
/// energy. Positive rates leave the litter; negative rates enter it.
pub fn finalize_litter_vapor(
    raw: RawLitterVapor,
    finalized: FinalizedLitterVapor,
    beginning: BeginningLitterPhaseState,
    phase_free_temperature_k: f64,
    interval_s: f64,
) -> Result<LitterVaporReceipt, LandSurfaceEnergyError> {
    finite_positive(interval_s, "vapor interval")?;
    validate_final_component(
        raw.raw_liquid_signed_rate_kg_m2_s,
        finalized.liquid_signed_rate_kg_m2_s,
        beginning.liquid_kg_m2_tile,
        interval_s,
    )?;
    validate_final_component(
        raw.raw_ice_signed_rate_kg_m2_s,
        finalized.ice_signed_rate_kg_m2_s,
        beginning.ice_kg_m2_tile,
        interval_s,
    )?;
    let liquid_mass = finalized.liquid_signed_rate_kg_m2_s * interval_s;
    let ice_mass = finalized.ice_signed_rate_kg_m2_s * interval_s;
    let liquid_specific = WATER_HEAT_CAPACITY_J_KG_K
        * (phase_free_temperature_k - REFERENCE_TEMPERATURE_K)
        + vaporization_enthalpy_j_kg(phase_free_temperature_k);
    let ice_specific = LITTER_ICE_HEAT_CAPACITY_J_KG_K
        * (phase_free_temperature_k - REFERENCE_TEMPERATURE_K)
        + sublimation_enthalpy_j_kg(phase_free_temperature_k);
    Ok(LitterVaporReceipt {
        raw,
        finalized,
        liquid_signed_mass_kg_m2: liquid_mass,
        ice_signed_mass_kg_m2: ice_mass,
        liquid_specific_enthalpy_j_kg: liquid_specific,
        ice_specific_enthalpy_j_kg: ice_specific,
        liquid_signed_energy_j_m2: liquid_mass * liquid_specific,
        ice_signed_energy_j_m2: ice_mass * ice_specific,
    })
}

pub fn install_finalized_vapor(
    configuration: LitterPhaseConfiguration,
    beginning: BeginningLitterPhaseState,
    phase_free_temperature_k: f64,
    vapor: LitterVaporReceipt,
) -> Result<PostVaporLitterState, LandSurfaceEnergyError> {
    let liquid = beginning.liquid_kg_m2_tile - vapor.liquid_signed_mass_kg_m2;
    let ice = beginning.ice_kg_m2_tile - vapor.ice_signed_mass_kg_m2;
    if !liquid.is_finite()
        || !ice.is_finite()
        || liquid < 0.0
        || ice < 0.0
        || ice > configuration.ice_capacity_kg_m2_tile
    {
        return Err(LandSurfaceEnergyError::FrozenLitterVapor(
            "post-vapor phase store domain",
        ));
    }
    let capacity = configuration.dry_heat_capacity_j_m2_k
        + liquid * WATER_HEAT_CAPACITY_J_KG_K
        + ice * LITTER_ICE_HEAT_CAPACITY_J_KG_K;
    Ok(PostVaporLitterState {
        liquid_kg_m2_tile: liquid,
        ice_kg_m2_tile: ice,
        sensible_energy_j_m2_tile: capacity * (phase_free_temperature_k - REFERENCE_TEMPERATURE_K),
        temperature_k: phase_free_temperature_k,
    })
}

/// Apply one bounded kinetic phase transfer. This function never invokes the
/// V2 nonlinear solver and therefore cannot cause a same-support re-solve.
pub fn apply_bounded_litter_phase(
    configuration: LitterPhaseConfiguration,
    post_vapor: PostVaporLitterState,
    interval_s: f64,
) -> Result<(LitterPhaseTransfer, EndingLitterPhaseState), LandSurfaceEnergyError> {
    finite_positive(interval_s, "phase interval")?;
    for (value, detail) in [
        (post_vapor.liquid_kg_m2_tile, "post-vapor liquid"),
        (post_vapor.ice_kg_m2_tile, "post-vapor ice"),
        (post_vapor.sensible_energy_j_m2_tile, "post-vapor energy"),
        (post_vapor.temperature_k, "post-vapor temperature"),
    ] {
        finite(value, detail)?;
    }
    if post_vapor.liquid_kg_m2_tile < 0.0
        || post_vapor.ice_kg_m2_tile < 0.0
        || post_vapor.ice_kg_m2_tile > configuration.ice_capacity_kg_m2_tile
    {
        return Err(LandSurfaceEnergyError::FrozenLitterPhaseClosure(
            "post-vapor phase domain",
        ));
    }
    let warm_bound = LITTER_ICE_DENSITY_KG_M3
        * LITTER_ICE_HEAT_CAPACITY_J_KG_K
        * configuration.litter_depth_m
        * (post_vapor.temperature_k - REFERENCE_TEMPERATURE_K).max(0.0)
        / LITTER_FUSION_ENTHALPY_J_KG;
    let cold_bound = LITTER_ICE_DENSITY_KG_M3
        * LITTER_ICE_HEAT_CAPACITY_J_KG_K
        * configuration.litter_depth_m
        * (REFERENCE_TEMPERATURE_K - post_vapor.temperature_k).max(0.0)
        / LITTER_FUSION_ENTHALPY_J_KG;
    let melt = post_vapor
        .ice_kg_m2_tile
        .min((interval_s / LITTER_ICE_TIMESCALE_S) * warm_bound.min(post_vapor.ice_kg_m2_tile));
    let freeze = post_vapor
        .liquid_kg_m2_tile
        .min(configuration.ice_capacity_kg_m2_tile - post_vapor.ice_kg_m2_tile)
        .min((interval_s / LITTER_ICE_TIMESCALE_S) * cold_bound.min(post_vapor.liquid_kg_m2_tile));
    let signed_phase = freeze - melt;
    let fusion = LITTER_FUSION_ENTHALPY_J_KG * signed_phase;
    let liquid = post_vapor.liquid_kg_m2_tile - freeze + melt;
    let ice = post_vapor.ice_kg_m2_tile + freeze - melt;
    let energy = post_vapor.sensible_energy_j_m2_tile + fusion;
    let capacity = configuration.dry_heat_capacity_j_m2_k
        + liquid * WATER_HEAT_CAPACITY_J_KG_K
        + ice * LITTER_ICE_HEAT_CAPACITY_J_KG_K;
    if !(capacity.is_finite() && capacity > 0.0) {
        return Err(LandSurfaceEnergyError::FrozenLitterPhaseClosure(
            "ending heat capacity",
        ));
    }
    let temperature = REFERENCE_TEMPERATURE_K + energy / capacity;
    if !(200.0..=350.0).contains(&temperature) {
        return Err(LandSurfaceEnergyError::FrozenLitterPhaseClosure(
            "ending phase temperature",
        ));
    }
    Ok((
        LitterPhaseTransfer {
            warm_energy_mass_bound_kg_m2: warm_bound,
            cold_energy_mass_bound_kg_m2: cold_bound,
            melt_kg_m2: melt,
            freeze_kg_m2: freeze,
            signed_phase_kg_m2: signed_phase,
            fusion_energy_j_m2: fusion,
        },
        EndingLitterPhaseState {
            liquid_kg_m2_tile: liquid,
            ice_kg_m2_tile: ice,
            sensible_energy_j_m2_tile: energy,
            temperature_k: temperature,
            heat_capacity_j_m2_k: capacity,
        },
    ))
}

#[cfg(test)]
#[allow(clippy::float_cmp)]
mod tests {
    use super::*;

    include!("litter_phase_tests.rs");
}
