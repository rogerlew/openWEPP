//! Deterministic nonlinear solve for the admitted snow-free surface system.
//!
//! The open-tile reduction is complete here.  Covered columns use the same
//! [`solve_normalized_system`] engine through an exact residual callback so
//! canopy V8 and ground unknowns remain one current-trial system without a
//! dependency cycle between the vegetation and LSE crates.

// The solver retains canonical equation symbols, exact branch comparisons,
// and auditable monolithic residual ordering from the authority oracle.
#![allow(
    clippy::float_cmp,
    clippy::large_enum_variant,
    clippy::many_single_char_names,
    clippy::missing_errors_doc,
    clippy::needless_range_loop,
    clippy::similar_names,
    clippy::too_many_lines
)]

use crate::covered_liquid::{
    CoveredLiquidPass, CoveredLiquidPreparation, finalize_covered_liquid, prepare_covered_liquid,
};
use crate::covered_output::{CoveredColumnEvaluation, CoveredOccupancyEvaluation};
use crate::numerics::{
    MAX_BACKTRACKING_HALVINGS, MAX_NEWTON_ITERATIONS, is_strict_residual_decrease,
    normalized_infinity_norm, solve_linear,
};
pub use crate::numerics::{
    NormalizedSolveOutcome, NumericalFailure, NumericalFailureKind, solve_normalized_system,
};
use crate::physics::{
    AIR_HEAT_CAPACITY_J_KG_K, BandDirectionalFluxes, BareSoilVaporOperands, BareSoilVaporResult,
    DRY_AIR_GAS_CONSTANT_J_KG_K, NeutralResistances, OpenNeutralGeometry, REFERENCE_TEMPERATURE_K,
    STEFAN_BOLTZMANN_W_M2_K4, WATER_HEAT_CAPACITY_J_KG_K, bare_soil_vapor, energy_tolerance,
    harmonic_interface_conductance_w_m2_k, litter_relative_humidity, open_neutral_resistances,
    partition_ground_shortwave, saturation_specific_humidity, vapor_export_w_m2,
};
use crate::{LandSurfaceEnergyError, NormalizedResidual, ResidualUnit, StepNorms};
use serde::Serialize;
use std::collections::{BTreeMap, BTreeSet};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SurfaceStorageBranch {
    FiniteCapacity,
    EquilibriumZero,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SurfaceClassKind {
    BareMineralSoil,
    ForestLitter,
}

#[derive(Clone, Debug, PartialEq)]
pub struct SoilThermalNodeOperands {
    pub layer_id: String,
    pub depth_m: f64,
    pub conductivity_w_m_k: f64,
    pub heat_capacity_j_m2_k: f64,
    pub beginning_temperature_k: f64,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct BareSoilParameters {
    pub top_layer_liquid_kg_m2: f64,
    pub top_layer_ice_kg_m2: f64,
    pub porosity: f64,
    pub saturated_matric_potential_mm: f64,
    pub clapp_hornberger_b: f64,
    pub theta_initial: f64,
}

#[derive(Clone, Debug, PartialEq)]
pub struct OpenSurfaceProblem {
    pub interval_s: f64,
    pub tile_fraction: f64,
    pub class: SurfaceClassKind,
    pub storage_branch: SurfaceStorageBranch,
    pub terminal_shortwave_w_m2_tile: BandDirectionalFluxes,
    pub surface_vis_albedo: f64,
    pub surface_nir_albedo: f64,
    pub surface_emissivity: f64,
    pub surface_depth_m: f64,
    pub surface_conductivity_w_m_k: f64,
    pub surface_dry_heat_capacity_j_m2_k: f64,
    pub litter_capacity_kg_m2_tile: Option<f64>,
    pub open_geometry: OpenNeutralGeometry,
    pub air_temperature_k: f64,
    pub air_specific_humidity_kg_kg: f64,
    pub air_pressure_pa: f64,
    pub reference_wind_m_s: f64,
    pub atmospheric_downward_longwave_w_m2: f64,
    pub surface_liquid_kg_m2_tile: f64,
    pub surface_enthalpy_j_m2_tile: f64,
    pub surface_temperature_warm_start_k: f64,
    pub bare_soil: Option<BareSoilParameters>,
    pub soil_nodes: Vec<SoilThermalNodeOperands>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize)]
pub enum WaterBranch {
    ConstitutiveLaw,
    AuthorizationActiveOrTie,
    Condensation,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct GroundWaterFlux {
    pub law_kg_m2_tile_s: f64,
    pub final_kg_m2_tile_s: f64,
    pub request_kg_m2_stand_ground: f64,
    pub authorization_kg_m2_stand_ground: Option<f64>,
    pub finalized_use_kg_m2_stand_ground: f64,
    pub condensation_credit_kg_m2_stand_ground: f64,
    pub branch: WaterBranch,
}

#[derive(Clone, Debug, PartialEq)]
pub struct SoilThermalResidual {
    pub layer_id: String,
    pub incoming_cn_w_m2: f64,
    pub outgoing_cn_w_m2: f64,
    pub storage_w_m2: f64,
    pub residual_w_m2: f64,
}

#[derive(Clone, Debug, PartialEq)]
pub struct OpenSurfaceEvaluation {
    pub raw_residuals: Vec<f64>,
    pub normalized_residuals: Vec<f64>,
    pub tolerances: Vec<f64>,
    pub surface_temperature_k: f64,
    pub soil_temperature_k: Vec<f64>,
    pub shortwave_absorbed_w_m2_tile: BandDirectionalFluxes,
    pub shortwave_reflected_w_m2_tile: BandDirectionalFluxes,
    pub longwave_net_w_m2_tile: f64,
    pub sensible_w_m2_tile: f64,
    pub vapor_energy_w_m2_tile: f64,
    pub surface_storage_w_m2_tile: f64,
    pub ending_surface_enthalpy_j_m2_tile: f64,
    pub ground_heat_cn_w_m2_tile: Vec<f64>,
    pub soil_thermal: Vec<SoilThermalResidual>,
    pub water: GroundWaterFlux,
    pub neutral_resistances: NeutralResistances,
    pub bare_soil_vapor: Option<BareSoilVaporResult>,
}

fn diagnostic_residual(
    identity: String,
    raw: f64,
    tolerance: f64,
    normalized: f64,
    unit: ResidualUnit,
) -> NormalizedResidual {
    NormalizedResidual {
        identity,
        raw,
        scale: raw.abs().max(1.0),
        tolerance,
        normalized,
        unit,
    }
}

pub(crate) fn open_failure_residuals(
    beginning: &OpenSurfaceProblem,
    detail: &OpenSurfaceEvaluation,
) -> Vec<NormalizedResidual> {
    detail
        .raw_residuals
        .iter()
        .zip(&detail.tolerances)
        .zip(&detail.normalized_residuals)
        .enumerate()
        .map(|(index, ((raw, tolerance), normalized))| {
            let identity = if index == 0 {
                "ground_surface_energy".to_owned()
            } else {
                format!("soil_thermal:{}", beginning.soil_nodes[index - 1].layer_id)
            };
            diagnostic_residual(
                identity,
                *raw,
                *tolerance,
                *normalized,
                ResidualUnit::WattsPerSquareMeter,
            )
        })
        .collect()
}

fn open_failure_bounds(
    problem: &OpenSurfaceProblem,
    failed_solution: &[f64],
    authorization_cap_rate: Option<f64>,
) -> Vec<String> {
    let mut bounds = failed_solution
        .iter()
        .enumerate()
        .flat_map(|(index, value)| {
            let identity = if index == 0 {
                "ground_surface_temperature_k".to_owned()
            } else {
                format!(
                    "soil_temperature_k:{}",
                    problem.soil_nodes[index - 1].layer_id
                )
            };
            [(200.0, "lower"), (350.0, "upper")]
                .into_iter()
                .filter(move |(bound, _)| value == bound)
                .map(move |(_, side)| format!("{identity}:{side}"))
        })
        .collect::<Vec<_>>();
    if problem.air_specific_humidity_kg_kg == 0.0 {
        bounds.push("air_specific_humidity_kg_kg:lower".into());
    }
    if problem.surface_liquid_kg_m2_tile == 0.0 {
        bounds.push("surface_liquid_kg_m2_tile:lower".into());
    }
    if authorization_cap_rate == Some(0.0) {
        bounds.push("ground_water_authorization_cap_rate:lower".into());
    }
    bounds
}

pub(crate) fn covered_failure_metadata(
    beginning: &CoveredColumnInputs,
    detail: &CoveredColumnEvaluation,
    failed_solution: &[f64],
) -> (Option<String>, Vec<String>) {
    let occupancy_id =
        (beginning.occupancies.len() == 1).then(|| beginning.occupancies[0].occupancy_id.clone());
    let mut active_bounds = detail
        .occupancies
        .iter()
        .filter(|occupancy| occupancy.wet_branch == WaterBranch::AuthorizationActiveOrTie)
        .map(|occupancy| {
            let identity = occupancy
                .source_water
                .first()
                .map_or("whole_column", |source| source.occupancy_id.as_str());
            if beginning.occupancies.len() == 1 {
                "canopy_liquid_store_cap".to_owned()
            } else {
                format!("{identity}:canopy_liquid_store_cap")
            }
        })
        .collect::<Vec<_>>();
    let mut record = |identity: String, value: f64, lower: f64, upper: f64| {
        if value == lower {
            active_bounds.push(format!("{identity}:lower"));
        }
        if value == upper {
            active_bounds.push(format!("{identity}:upper"));
        }
    };
    for (index, occupancy) in beginning.occupancies.iter().enumerate() {
        let block = &failed_solution[index * 10..(index + 1) * 10];
        let prefix = &occupancy.occupancy_id;
        record(format!("{prefix}:beta_sun"), block[4], 0.0, 1.0);
        record(format!("{prefix}:beta_shade"), block[5], 0.0, 1.0);
        for (offset, component) in [
            "sun_leaf_temperature_k",
            "shade_leaf_temperature_k",
            "wet_surface_temperature_k",
            "dry_stem_temperature_k",
        ]
        .iter()
        .enumerate()
        {
            record(
                format!("{prefix}:{component}"),
                block[6 + offset],
                200.0,
                350.0,
            );
        }
    }
    let common = &failed_solution[10 * beginning.occupancies.len()..];
    record(
        "shared_canopy_air_temperature_k".into(),
        common[0],
        200.0,
        350.0,
    );
    record(
        "shared_canopy_air_specific_humidity_kg_kg".into(),
        common[1],
        0.0,
        0.1,
    );
    record(
        "ground_surface_temperature_k".into(),
        common[2],
        200.0,
        350.0,
    );
    for (temperature, node) in common[3..].iter().zip(&beginning.ground.soil_nodes) {
        record(
            format!("soil_temperature_k:{}", node.layer_id),
            *temperature,
            200.0,
            350.0,
        );
    }
    (occupancy_id, active_bounds)
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct FrozenActiveBranches {
    pub ground: WaterBranch,
}

impl OpenSurfaceProblem {
    fn validate(&self) -> Result<(), LandSurfaceEnergyError> {
        for (value, field) in [
            (self.interval_s, "interval_s"),
            (self.tile_fraction, "tile_fraction"),
            (self.surface_depth_m, "surface_depth_m"),
            (
                self.surface_conductivity_w_m_k,
                "surface_conductivity_w_m_k",
            ),
            (self.air_temperature_k, "air_temperature_k"),
            (self.air_pressure_pa, "air_pressure_pa"),
            (self.reference_wind_m_s, "reference_wind_m_s"),
        ] {
            if !value.is_finite() {
                return Err(LandSurfaceEnergyError::NonFinite(field));
            }
            if value <= 0.0 {
                return Err(LandSurfaceEnergyError::ConstitutiveDomain(field));
            }
        }
        if self.tile_fraction > 1.0 {
            return Err(LandSurfaceEnergyError::ConstitutiveDomain("tile_fraction"));
        }
        for (value, field) in [
            (
                self.air_specific_humidity_kg_kg,
                "air_specific_humidity_kg_kg",
            ),
            (
                self.atmospheric_downward_longwave_w_m2,
                "atmospheric_downward_longwave_w_m2",
            ),
            (self.surface_liquid_kg_m2_tile, "surface_liquid_kg_m2_tile"),
        ] {
            if !value.is_finite() {
                return Err(LandSurfaceEnergyError::NonFinite(field));
            }
            if value < 0.0 {
                return Err(LandSurfaceEnergyError::ConstitutiveDomain(field));
            }
        }
        if self.surface_emissivity != 1.0 {
            return Err(LandSurfaceEnergyError::UnsupportedDomain(
                "surface_emissivity_must_equal_one",
            ));
        }
        if self.soil_nodes.is_empty() {
            return Err(LandSurfaceEnergyError::topology_cardinality("soil_nodes"));
        }
        for node in &self.soil_nodes {
            if node.layer_id.is_empty() {
                return Err(LandSurfaceEnergyError::topology_cardinality(
                    "soil_layer_id",
                ));
            }
            for (value, field) in [
                (node.depth_m, "soil_depth_m"),
                (node.conductivity_w_m_k, "soil_conductivity_w_m_k"),
                (node.heat_capacity_j_m2_k, "soil_heat_capacity_j_m2_k"),
                (node.beginning_temperature_k, "soil_temperature_k"),
            ] {
                if !value.is_finite() {
                    return Err(LandSurfaceEnergyError::NonFinite(field));
                }
                if value <= 0.0 {
                    return Err(LandSurfaceEnergyError::ConstitutiveDomain(field));
                }
            }
        }
        match self.class {
            SurfaceClassKind::BareMineralSoil => {
                if self.bare_soil.is_none() || self.litter_capacity_kg_m2_tile.is_some() {
                    return Err(LandSurfaceEnergyError::ConstitutiveDomain(
                        "bare_soil_class_fields",
                    ));
                }
            }
            SurfaceClassKind::ForestLitter => {
                if self.bare_soil.is_some() || self.litter_capacity_kg_m2_tile.is_none() {
                    return Err(LandSurfaceEnergyError::ConstitutiveDomain(
                        "forest_litter_class_fields",
                    ));
                }
            }
        }
        match self.storage_branch {
            SurfaceStorageBranch::FiniteCapacity => {
                let capacity = self.surface_dry_heat_capacity_j_m2_k
                    + self.surface_liquid_kg_m2_tile * WATER_HEAT_CAPACITY_J_KG_K;
                if !capacity.is_finite() || capacity <= 0.0 {
                    return Err(LandSurfaceEnergyError::ConstitutiveDomain(
                        "finite_surface_capacity",
                    ));
                }
            }
            SurfaceStorageBranch::EquilibriumZero => {
                if self.surface_dry_heat_capacity_j_m2_k != 0.0
                    || self.surface_liquid_kg_m2_tile != 0.0
                    || self.surface_enthalpy_j_m2_tile != 0.0
                {
                    return Err(LandSurfaceEnergyError::ConstitutiveDomain(
                        "equilibrium_zero_surface_state",
                    ));
                }
            }
        }
        Ok(())
    }

    #[must_use]
    pub fn initial_trial(&self) -> Vec<f64> {
        let mut result = Vec::with_capacity(self.soil_nodes.len() + 1);
        result.push(self.surface_temperature_warm_start_k);
        result.extend(
            self.soil_nodes
                .iter()
                .map(|node| node.beginning_temperature_k),
        );
        result
    }
}

fn temperature_trial_is_valid(trial: &[f64]) -> bool {
    trial
        .iter()
        .all(|value| value.is_finite() && (200.0..=350.0).contains(value))
}

pub fn evaluate_open_surface(
    problem: &OpenSurfaceProblem,
    trial: &[f64],
    authorization_cap_rate_kg_m2_tile_s: Option<f64>,
    frozen: Option<FrozenActiveBranches>,
) -> Result<OpenSurfaceEvaluation, LandSurfaceEnergyError> {
    problem.validate()?;
    if trial.len() != problem.soil_nodes.len() + 1 {
        return Err(LandSurfaceEnergyError::topology_domain("open_trial_shape"));
    }
    if !temperature_trial_is_valid(trial) {
        return Err(LandSurfaceEnergyError::ConstitutiveDomain(
            "open_temperature_bounds",
        ));
    }
    if let Some(cap) = authorization_cap_rate_kg_m2_tile_s {
        if !cap.is_finite() {
            return Err(LandSurfaceEnergyError::NonFinite("authorization_cap_rate"));
        }
        if cap < 0.0 {
            return Err(LandSurfaceEnergyError::water_bound(
                "negative_authorization_cap_rate",
            ));
        }
    }
    let surface_temperature = trial[0];
    let soil_temperatures = &trial[1..];
    let resistance = open_neutral_resistances(problem.open_geometry, problem.reference_wind_m_s)?;
    let air_density =
        problem.air_pressure_pa / (DRY_AIR_GAS_CONSTANT_J_KG_K * problem.air_temperature_k);
    let (law, bare_detail) = if problem.class == SurfaceClassKind::BareMineralSoil
        && problem.surface_liquid_kg_m2_tile == 0.0
    {
        let parameters = problem
            .bare_soil
            .ok_or(LandSurfaceEnergyError::ConstitutiveDomain(
                "missing_bare_soil_parameters",
            ))?;
        let detail = bare_soil_vapor(BareSoilVaporOperands {
            top_layer_liquid_kg_m2: parameters.top_layer_liquid_kg_m2,
            top_layer_ice_kg_m2: parameters.top_layer_ice_kg_m2,
            top_layer_depth_m: problem.soil_nodes[0].depth_m,
            porosity: parameters.porosity,
            saturated_matric_potential_mm: parameters.saturated_matric_potential_mm,
            clapp_hornberger_b: parameters.clapp_hornberger_b,
            theta_initial: parameters.theta_initial,
            surface_temperature_k: surface_temperature,
            recipient_specific_humidity_kg_kg: problem.air_specific_humidity_kg_kg,
            pressure_pa: problem.air_pressure_pa,
            aerodynamic_vapor_resistance_s_m: resistance.vapor_s_m,
            moist_air_density_kg_m3: air_density,
        })?;
        (detail.signed_flux_kg_m2_s, Some(detail))
    } else {
        let relative_humidity = match problem.class {
            SurfaceClassKind::BareMineralSoil => 1.0,
            SurfaceClassKind::ForestLitter => litter_relative_humidity(
                problem.surface_liquid_kg_m2_tile,
                problem.litter_capacity_kg_m2_tile.ok_or(
                    LandSurfaceEnergyError::ConstitutiveDomain("litter_capacity"),
                )?,
            )?,
        };
        let saturated = saturation_specific_humidity(surface_temperature, problem.air_pressure_pa)?;
        let surface_q = relative_humidity * saturated
            + (1.0 - relative_humidity) * problem.air_specific_humidity_kg_kg;
        (
            air_density * (surface_q - problem.air_specific_humidity_kg_kg) / resistance.vapor_s_m,
            None,
        )
    };
    let request_rate = law.max(0.0);
    let natural_branch = if law < 0.0 {
        WaterBranch::Condensation
    } else if authorization_cap_rate_kg_m2_tile_s.is_some_and(|cap| cap <= law) {
        WaterBranch::AuthorizationActiveOrTie
    } else {
        WaterBranch::ConstitutiveLaw
    };
    let branch = frozen.map_or(natural_branch, |value| value.ground);
    let final_vapor = match branch {
        WaterBranch::AuthorizationActiveOrTie => authorization_cap_rate_kg_m2_tile_s.ok_or(
            LandSurfaceEnergyError::water_cardinality("frozen_cap_without_authorization"),
        )?,
        WaterBranch::ConstitutiveLaw | WaterBranch::Condensation => law,
    };
    let uses_surface_store = !(problem.class == SurfaceClassKind::BareMineralSoil
        && problem.surface_liquid_kg_m2_tile == 0.0);
    let ending_pre_ingress_water = if uses_surface_store {
        problem.surface_liquid_kg_m2_tile - final_vapor.max(0.0) * problem.interval_s
            + (-final_vapor).max(0.0) * problem.interval_s
    } else {
        problem.surface_liquid_kg_m2_tile
    };
    if ending_pre_ingress_water < -1.0e-14 {
        return Err(LandSurfaceEnergyError::water_bound(
            "surface_water_negative_after_finalized_vapor",
        ));
    }
    let shortwave = partition_ground_shortwave(
        problem.terminal_shortwave_w_m2_tile,
        problem.surface_vis_albedo,
        problem.surface_nir_albedo,
    )?;
    let shortwave_absorbed = shortwave.absorbed.total();
    let longwave = problem.atmospheric_downward_longwave_w_m2
        - problem.surface_emissivity * STEFAN_BOLTZMANN_W_M2_K4 * surface_temperature.powi(4);
    let sensible =
        air_density * AIR_HEAT_CAPACITY_J_KG_K * (surface_temperature - problem.air_temperature_k)
            / resistance.heat_s_m;
    let vapor_energy = vapor_export_w_m2(final_vapor, surface_temperature)?;
    let (storage, ending_enthalpy, beginning_surface) = match problem.storage_branch {
        SurfaceStorageBranch::FiniteCapacity => {
            let ending_capacity = problem.surface_dry_heat_capacity_j_m2_k
                + ending_pre_ingress_water.max(0.0) * WATER_HEAT_CAPACITY_J_KG_K;
            if ending_capacity <= 0.0 {
                return Err(LandSurfaceEnergyError::ConstitutiveDomain(
                    "finite_surface_ending_capacity",
                ));
            }
            let ending = ending_capacity * (surface_temperature - REFERENCE_TEMPERATURE_K);
            let beginning_capacity = problem.surface_dry_heat_capacity_j_m2_k
                + problem.surface_liquid_kg_m2_tile * WATER_HEAT_CAPACITY_J_KG_K;
            (
                (ending - problem.surface_enthalpy_j_m2_tile) / problem.interval_s,
                ending,
                REFERENCE_TEMPERATURE_K + problem.surface_enthalpy_j_m2_tile / beginning_capacity,
            )
        }
        SurfaceStorageBranch::EquilibriumZero => (0.0, 0.0, surface_temperature),
    };
    let first = &problem.soil_nodes[0];
    let surface_conductance = harmonic_interface_conductance_w_m2_k(
        problem.surface_depth_m,
        problem.surface_conductivity_w_m_k,
        first.depth_m,
        first.conductivity_w_m_k,
    )?;
    let mut beginning_fluxes = Vec::with_capacity(problem.soil_nodes.len());
    let mut ending_fluxes = Vec::with_capacity(problem.soil_nodes.len());
    beginning_fluxes
        .push(surface_conductance * (beginning_surface - first.beginning_temperature_k));
    ending_fluxes.push(surface_conductance * (surface_temperature - soil_temperatures[0]));
    for index in 0..problem.soil_nodes.len().saturating_sub(1) {
        let upper = &problem.soil_nodes[index];
        let lower = &problem.soil_nodes[index + 1];
        let conductance = harmonic_interface_conductance_w_m2_k(
            upper.depth_m,
            upper.conductivity_w_m_k,
            lower.depth_m,
            lower.conductivity_w_m_k,
        )?;
        beginning_fluxes
            .push(conductance * (upper.beginning_temperature_k - lower.beginning_temperature_k));
        ending_fluxes.push(conductance * (soil_temperatures[index] - soil_temperatures[index + 1]));
    }
    let cn_fluxes: Vec<f64> = beginning_fluxes
        .iter()
        .zip(ending_fluxes.iter())
        .map(|(beginning, ending)| 0.5 * (beginning + ending))
        .collect();
    let surface_operands = [
        shortwave_absorbed,
        longwave,
        -sensible,
        -vapor_energy,
        -cn_fluxes[0],
        -storage,
    ];
    let mut residuals = vec![surface_operands.iter().sum()];
    let mut scales = vec![
        surface_operands
            .iter()
            .map(|value| value.abs())
            .sum::<f64>()
            .max(1.0),
    ];
    let mut soil_records = Vec::with_capacity(problem.soil_nodes.len());
    for (index, node) in problem.soil_nodes.iter().enumerate() {
        let incoming = cn_fluxes[index];
        let outgoing = cn_fluxes.get(index + 1).copied().unwrap_or(0.0);
        let node_storage = node.heat_capacity_j_m2_k
            * (soil_temperatures[index] - node.beginning_temperature_k)
            / problem.interval_s;
        let residual = incoming - outgoing - node_storage;
        residuals.push(residual);
        scales.push((incoming.abs() + outgoing.abs() + node_storage.abs()).max(1.0));
        soil_records.push(SoilThermalResidual {
            layer_id: node.layer_id.clone(),
            incoming_cn_w_m2: incoming,
            outgoing_cn_w_m2: outgoing,
            storage_w_m2: node_storage,
            residual_w_m2: residual,
        });
    }
    let tolerances: Vec<f64> = scales.iter().copied().map(energy_tolerance).collect();
    let normalized = residuals
        .iter()
        .zip(tolerances.iter())
        .map(|(residual, tolerance)| residual / tolerance)
        .collect();
    let authorization_amount = authorization_cap_rate_kg_m2_tile_s
        .map(|cap| cap * problem.tile_fraction * problem.interval_s);
    let finalized_use = if branch == WaterBranch::AuthorizationActiveOrTie {
        authorization_amount.ok_or(LandSurfaceEnergyError::water_cardinality(
            "missing_active_authorization",
        ))?
    } else {
        final_vapor.max(0.0) * problem.tile_fraction * problem.interval_s
    };
    Ok(OpenSurfaceEvaluation {
        raw_residuals: residuals,
        normalized_residuals: normalized,
        tolerances,
        surface_temperature_k: surface_temperature,
        soil_temperature_k: soil_temperatures.to_vec(),
        shortwave_absorbed_w_m2_tile: shortwave.absorbed,
        shortwave_reflected_w_m2_tile: shortwave.reflected,
        longwave_net_w_m2_tile: longwave,
        sensible_w_m2_tile: sensible,
        vapor_energy_w_m2_tile: vapor_energy,
        surface_storage_w_m2_tile: storage,
        ending_surface_enthalpy_j_m2_tile: ending_enthalpy,
        ground_heat_cn_w_m2_tile: cn_fluxes,
        soil_thermal: soil_records,
        water: GroundWaterFlux {
            law_kg_m2_tile_s: law,
            final_kg_m2_tile_s: final_vapor,
            request_kg_m2_stand_ground: request_rate * problem.tile_fraction * problem.interval_s,
            authorization_kg_m2_stand_ground: authorization_amount,
            finalized_use_kg_m2_stand_ground: finalized_use,
            condensation_credit_kg_m2_stand_ground: (-final_vapor).max(0.0)
                * problem.tile_fraction
                * problem.interval_s,
            branch,
        },
        neutral_resistances: resistance,
        bare_soil_vapor: bare_detail,
    })
}

#[derive(Clone, Debug, PartialEq)]
pub struct OpenSurfaceCandidate {
    pub surface_enthalpy_j_m2_tile: f64,
    pub surface_temperature_warm_start_k: f64,
    pub soil_temperature_k: Vec<f64>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct AcceptedOpenSurface {
    pub solution: Vec<f64>,
    pub evaluation: OpenSurfaceEvaluation,
    pub candidate: OpenSurfaceCandidate,
    pub iterations: u32,
    pub backtracking_count: u32,
    pub step_norm: f64,
}

#[derive(Clone, Debug, PartialEq)]
pub enum OpenSurfaceSolveOutcome {
    Accepted(AcceptedOpenSurface),
    Rejected(NumericalFailure),
}

pub fn solve_open_surface(
    beginning: &OpenSurfaceProblem,
    authorization_cap_rate_kg_m2_tile_s: Option<f64>,
    initial_trial: Option<Vec<f64>>,
) -> Result<OpenSurfaceSolveOutcome, LandSurfaceEnergyError> {
    beginning.validate()?;
    let initial = initial_trial.unwrap_or_else(|| beginning.initial_trial());
    let result = solve_normalized_system(
        |trial, frozen: Option<&FrozenActiveBranches>| {
            let detail = evaluate_open_surface(
                beginning,
                trial,
                authorization_cap_rate_kg_m2_tile_s,
                frozen.copied(),
            )?;
            Ok((detail.normalized_residuals.clone(), detail))
        },
        initial,
        &vec![1.0; beginning.soil_nodes.len() + 1],
        temperature_trial_is_valid,
        |detail: &OpenSurfaceEvaluation| FrozenActiveBranches {
            ground: detail.water.branch,
        },
    )?;
    Ok(match result {
        NormalizedSolveOutcome::Accepted {
            solution,
            detail,
            iterations,
            backtracking_count,
            step_norm,
            ..
        } => OpenSurfaceSolveOutcome::Accepted(AcceptedOpenSurface {
            solution,
            candidate: OpenSurfaceCandidate {
                surface_enthalpy_j_m2_tile: detail.ending_surface_enthalpy_j_m2_tile,
                surface_temperature_warm_start_k: detail.surface_temperature_k,
                soil_temperature_k: detail.soil_temperature_k.clone(),
            },
            evaluation: detail,
            iterations,
            backtracking_count,
            step_norm,
        }),
        NormalizedSolveOutcome::Rejected(mut failure) => {
            let detail = evaluate_open_surface(
                beginning,
                &failure.failed_solution,
                authorization_cap_rate_kg_m2_tile_s,
                None,
            )?;
            failure.ordered_residuals = open_failure_residuals(beginning, &detail);
            failure.active_bounds = open_failure_bounds(
                beginning,
                &failure.failed_solution,
                authorization_cap_rate_kg_m2_tile_s,
            );
            OpenSurfaceSolveOutcome::Rejected(failure)
        }
    })
}

// -------------------------------------------------------------------------
// Concrete covered-column constitutive block
// -------------------------------------------------------------------------

/// Frozen biochemical constants for one C3 sun or shade class.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct LeafBiochemicalInputs {
    pub leaf_area_m2_m2_tile: f64,
    pub absorbed_shortwave_w_m2_tile: f64,
    pub absorbed_par_w_m2_leaf: f64,
    pub vcmax25: f64,
    pub jmax25: f64,
    pub rd25: f64,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct BiochemicalConstants {
    pub ha_vcmax_j_mol: f64,
    pub hd_vcmax_j_mol: f64,
    pub entropy_vcmax_j_mol_k: f64,
    pub ha_jmax_j_mol: f64,
    pub hd_jmax_j_mol: f64,
    pub entropy_jmax_j_mol_k: f64,
    pub kc25_pa: f64,
    pub ha_kc_j_mol: f64,
    pub ko25_pa: f64,
    pub ha_ko_j_mol: f64,
    pub gamma25_pa: f64,
    pub ha_gamma_j_mol: f64,
    pub oxygen_partial_pressure_pa: f64,
    pub tp_vcmax_ratio: f64,
    pub electron_quantum_yield: f64,
    pub par_photon_umol_per_j: f64,
    pub electron_curvature: f64,
    pub ac_aj_curvature: f64,
    pub ag_ap_curvature: f64,
}

#[derive(Clone, Debug, PartialEq)]
pub struct RootHydraulicLayer {
    pub layer_id: String,
    pub accessible: bool,
    pub frozen: bool,
    pub root_fraction: f64,
    pub soil_potential_mm: f64,
    pub gravity_head_mm: f64,
    pub z3_m: f64,
    pub dxroot_m: f64,
    pub ksoil_m2_s: f64,
}

#[derive(Clone, Debug, PartialEq)]
pub struct SourceWaterCap {
    pub request_rate_kg_m2_tile_s: f64,
    pub authorization_rate_kg_m2_tile_s: f64,
}

#[derive(Clone, Debug, PartialEq)]
pub struct CoveredWaterCaps {
    /// Exact `(occupancy_id, layer_id) -> tile-ground rate` identity.
    pub root: BTreeMap<(String, String), SourceWaterCap>,
    /// The one ground-source authorization for this tile.
    pub ground: SourceWaterCap,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct CoveredFrozenBranches {
    pub root: BTreeMap<(String, String), WaterBranch>,
    pub wet: BTreeMap<String, WaterBranch>,
    pub ground: Option<WaterBranch>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct SourceWaterFlux {
    pub occupancy_id: String,
    pub layer_id: String,
    pub law_kg_m2_tile_s: f64,
    pub final_kg_m2_tile_s: f64,
    pub request_kg_m2_stand_ground: f64,
    pub authorization_kg_m2_stand_ground: Option<f64>,
    pub finalized_use_kg_m2_stand_ground: f64,
    pub branch: WaterBranch,
}

#[derive(Clone, Debug, PartialEq)]
pub struct CoveredOccupancyInputs {
    pub occupancy_id: String,
    pub medlyn_g1_kpa_sqrt: f64,
    pub g0_umol_m2_s: f64,
    pub sun: LeafBiochemicalInputs,
    pub shade: LeafBiochemicalInputs,
    pub biochemical: BiochemicalConstants,
    pub stem_area_m2_m2_tile: f64,
    pub stem_absorbed_shortwave_w_m2_tile: f64,
    /// Immutable beginning occupancy store before current top-to-bottom E04.
    pub beginning_canopy_liquid_kg_m2_tile: f64,
    pub liquid_interception_fraction: f64,
    pub liquid_capacity_kg_m2_plant: f64,
    pub stemflow_fraction: f64,
    pub gb_leaf_m_s: f64,
    pub gb_wet_m_s: f64,
    pub gb_stem_m_s: f64,
    pub lai: f64,
    pub sai: f64,
    pub clumping_index: f64,
    pub k1_sun_max_s1: f64,
    pub k1_shade_max_s1: f64,
    pub k2_max: f64,
    pub k3_max_m_s: f64,
    pub height_m: f64,
    pub root_to_leaf_area: f64,
    pub p50_leaf_mm: f64,
    pub p50_xylem_mm: f64,
    pub p50_root_mm: f64,
    pub vulnerability_exponent: f64,
    pub root_layers: Vec<RootHydraulicLayer>,
}

/// Bound E01--E03 band/direction absorption owned by one physical canopy
/// occupancy before wet/dry surface partitioning.
#[derive(Clone, Debug, PartialEq)]
pub struct CoveredOccupancyShortwaveInputs {
    pub occupancy_id: String,
    pub sun_leaf_absorbed_w_m2_tile: BandDirectionalFluxes,
    pub shade_leaf_absorbed_w_m2_tile: BandDirectionalFluxes,
    pub stem_absorbed_w_m2_tile: BandDirectionalFluxes,
}

/// Complete column shortwave boundary receipt from the admitted radiation
/// owner. These are primitive E01--E03 results, not inferred all-wave shares.
#[derive(Clone, Debug, PartialEq)]
pub struct CoveredColumnShortwaveInputs {
    pub incident_w_m2_tile: BandDirectionalFluxes,
    pub top_reflected_w_m2_tile: BandDirectionalFluxes,
    /// Ground absorption attributed to each incident band/direction after
    /// reciprocal ground-canopy reflection. This is distinct from the raw
    /// downward terminal flux at the ground boundary.
    pub ground_absorbed_by_incident_w_m2_tile: BandDirectionalFluxes,
    pub occupancies: Vec<CoveredOccupancyShortwaveInputs>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct CoveredColumnInputs {
    pub interval_s: f64,
    pub tile_fraction: f64,
    pub pressure_pa: f64,
    pub air_temperature_k: f64,
    pub air_specific_humidity_kg_kg: f64,
    pub reference_wind_m_s: f64,
    pub atmospheric_downward_longwave_w_m2: f64,
    pub ca_pa: f64,
    pub canopy_to_atmosphere_heat_resistance_s_m: f64,
    pub canopy_to_atmosphere_vapor_resistance_s_m: f64,
    pub latent_heat_j_kg: f64,
    /// Current interval rain entering the top occupancy on tile-ground basis.
    pub top_rain_kg_m2_tile: f64,
    pub under_canopy_geometry: crate::physics::UnderCanopyGeometry,
    pub ground: OpenSurfaceProblem,
    pub occupancies: Vec<CoveredOccupancyInputs>,
    pub shortwave: CoveredColumnShortwaveInputs,
}

#[derive(Clone, Copy, Debug, PartialEq)]
struct LeafTrialState {
    surface_q: f64,
    rs_s_m: f64,
    ci_pa: f64,
    gross_assimilation_umol_co2_m2_leaf_s: f64,
    net_assimilation_umol_co2_m2_leaf_s: f64,
    dark_respiration_umol_co2_m2_leaf_s: f64,
}

#[derive(Clone, Copy, Debug, PartialEq)]
struct LeafCarbonState {
    ag: f64,
    an: f64,
    rd: f64,
}

const MOLAR_GAS_CONSTANT: f64 = 8.314_462_618_153_24;

fn log_one_plus_exp(value: f64) -> f64 {
    if value > 0.0 {
        value + (-value).exp().ln_1p()
    } else {
        value.exp().ln_1p()
    }
}

fn arrhenius(temperature: f64, activation: f64) -> Result<f64, LandSurfaceEnergyError> {
    if !temperature.is_finite()
        || !activation.is_finite()
        || temperature <= 0.0
        || activation <= 0.0
    {
        return Err(LandSurfaceEnergyError::ConstitutiveDomain(
            "arrhenius_response",
        ));
    }
    Ok((activation / MOLAR_GAS_CONSTANT * (1.0 / 298.15 - 1.0 / temperature)).exp())
}

fn peaked(
    temperature: f64,
    activation: f64,
    deactivation: f64,
    entropy: f64,
) -> Result<f64, LandSurfaceEnergyError> {
    if [temperature, activation, deactivation, entropy]
        .iter()
        .any(|value| !value.is_finite() || *value <= 0.0)
    {
        return Err(LandSurfaceEnergyError::ConstitutiveDomain(
            "peaked_response",
        ));
    }
    let reference = 298.15;
    let log_factor = activation * (temperature - reference)
        / (MOLAR_GAS_CONSTANT * temperature * reference)
        + log_one_plus_exp((reference * entropy - deactivation) / (MOLAR_GAS_CONSTANT * reference))
        - log_one_plus_exp(
            (temperature * entropy - deactivation) / (MOLAR_GAS_CONSTANT * temperature),
        );
    let result = log_factor.exp();
    if result.is_finite() {
        Ok(result)
    } else {
        Err(LandSurfaceEnergyError::ConstitutiveDomain(
            "peaked_response",
        ))
    }
}

fn smaller_quadratic_root(a: f64, b: f64, c: f64) -> Result<f64, LandSurfaceEnergyError> {
    if a == 0.0 {
        return if b == 0.0 {
            Err(LandSurfaceEnergyError::ConstitutiveDomain("quadratic"))
        } else {
            Ok(-c / b)
        };
    }
    if c == 0.0 {
        return Ok(0.0_f64.min(-b / a));
    }
    let mut discriminant = b * b - 4.0 * a * c;
    if discriminant < 0.0 {
        let scale = (b * b).abs().max((4.0 * a * c).abs());
        if discriminant >= -64.0 * f64::EPSILON * scale {
            discriminant = 0.0;
        } else {
            return Err(LandSurfaceEnergyError::ConstitutiveDomain(
                "photosynthesis_discriminant",
            ));
        }
    }
    let root = discriminant.sqrt();
    let q = -0.5 * (b + root.copysign(b));
    Ok((q / a).min(c / q))
}

fn canopy_saturation_q(temperature: f64, pressure: f64) -> Result<f64, LandSurfaceEnergyError> {
    let tc = temperature - 273.15;
    if !(0.0..=100.0).contains(&tc) {
        return Err(LandSurfaceEnergyError::ConstitutiveDomain(
            "liquid_saturation_polynomial",
        ));
    }
    let coefficients = [
        6.112_134_76,
        4.440_078_56e-1,
        1.430_642_34e-2,
        2.644_614_37e-4,
        3.059_035_58e-6,
        1.962_372_41e-8,
        8.923_447_72e-11,
        -3.732_084_10e-13,
        2.093_399_97e-16,
    ];
    let es = 100.0
        * coefficients
            .iter()
            .scan(1.0, |power, value| {
                let term = value * *power;
                *power *= tc;
                Some(term)
            })
            .sum::<f64>();
    Ok(0.622 * es / (pressure - 0.378 * es))
}

#[allow(clippy::too_many_arguments)]
fn leaf_trial_state(
    inputs: LeafBiochemicalInputs,
    p: BiochemicalConstants,
    temperature: f64,
    qcan: f64,
    beta: f64,
    column: &CoveredColumnInputs,
    gb_leaf: f64,
    g0_umol_m2_s: f64,
    medlyn_g1_kpa_sqrt: f64,
) -> Result<LeafTrialState, LandSurfaceEnergyError> {
    let vcmax_factor = peaked(
        temperature,
        p.ha_vcmax_j_mol,
        p.hd_vcmax_j_mol,
        p.entropy_vcmax_j_mol_k,
    )?;
    let jmax_factor = peaked(
        temperature,
        p.ha_jmax_j_mol,
        p.hd_jmax_j_mol,
        p.entropy_jmax_j_mol_k,
    )?;
    let vcmax = inputs.vcmax25 * vcmax_factor;
    let jmax = inputs.jmax25 * jmax_factor;
    let kc = p.kc25_pa * arrhenius(temperature, p.ha_kc_j_mol)?;
    let ko = p.ko25_pa * arrhenius(temperature, p.ha_ko_j_mol)?;
    let gamma = p.gamma25_pa * arrhenius(temperature, p.ha_gamma_j_mol)?;
    let tp = p.tp_vcmax_ratio * inputs.vcmax25 * vcmax_factor;
    let rd = inputs.rd25 * peaked(temperature, 46_390.0, 150_650.0, 490.0)?;
    let qsurface = canopy_saturation_q(temperature, column.pressure_pa)?;
    let es_leaf = qsurface * column.pressure_pa / (0.622 + 0.378 * qsurface);
    let e_can = qcan * column.pressure_pa / (0.622 + 0.378 * qcan);
    let vpd = (es_leaf - e_can) / 1000.0;
    if vpd <= 0.0 {
        return Err(LandSurfaceEnergyError::ConstitutiveDomain("surface_vpd"));
    }
    let carbon_at_ci = |ci: f64| -> Result<LeafCarbonState, LandSurfaceEnergyError> {
        let ipsii = 0.5
            * p.electron_quantum_yield
            * p.par_photon_umol_per_j
            * inputs.absorbed_par_w_m2_leaf;
        let electron = if ipsii > 0.0 {
            smaller_quadratic_root(p.electron_curvature, -(ipsii + jmax), ipsii * jmax)?
        } else {
            0.0
        };
        let ac = vcmax * (ci - gamma) / (ci + kc * (1.0 + p.oxygen_partial_pressure_pa / ko));
        let aj = electron * (ci - gamma) / (4.0 * ci + 8.0 * gamma);
        let ai = smaller_quadratic_root(p.ac_aj_curvature, -(ac + aj), ac * aj)?;
        let ag = smaller_quadratic_root(p.ag_ap_curvature, -(ai + 3.0 * tp), ai * 3.0 * tp)?;
        let an = ag - rd;
        if !ag.is_finite() || !an.is_finite() || !rd.is_finite() {
            return Err(LandSurfaceEnergyError::ConstitutiveDomain(
                "photosynthesis_nonfinite",
            ));
        }
        Ok(LeafCarbonState { ag, an, rd })
    };
    let residual = |ci: f64| -> Result<(f64, f64), LandSurfaceEnergyError> {
        let carbon = carbon_at_ci(ci)?;
        let an = carbon.an;
        let rb = 1.0 / gb_leaf;
        let cs = column.ca_pa - 1.4 * rb * MOLAR_GAS_CONSTANT * temperature * an * 1.0e-6;
        if cs <= 0.0 {
            return Err(LandSurfaceEnergyError::ConstitutiveDomain("surface_co2"));
        }
        let potential = if an <= 0.0 {
            g0_umol_m2_s
        } else {
            g0_umol_m2_s
                + 1.6 * (1.0 + medlyn_g1_kpa_sqrt / vpd.sqrt()) * an / (cs / column.pressure_pa)
        };
        let gs = g0_umol_m2_s + beta * (potential - g0_umol_m2_s);
        let gs_ms = gs * 1.0e-6 * MOLAR_GAS_CONSTANT * temperature / column.pressure_pa;
        if gs_ms <= 0.0 {
            return Err(LandSurfaceEnergyError::ConstitutiveDomain(
                "stomatal_conductance",
            ));
        }
        let rs = 1.0 / gs_ms;
        let predicted =
            column.ca_pa - (1.4 * rb + 1.6 * rs) * MOLAR_GAS_CONSTANT * temperature * an * 1.0e-6;
        Ok((ci - predicted, rs))
    };
    let mut a = gamma;
    let mut b = column.ca_pa;
    let (mut fa, _) = residual(a)?;
    let (mut fb, mut rs) = residual(b)?;
    if fa == 0.0 {
        rs = residual(a)?.1;
        let carbon = carbon_at_ci(a)?;
        return Ok(LeafTrialState {
            surface_q: qsurface,
            rs_s_m: rs,
            ci_pa: a,
            gross_assimilation_umol_co2_m2_leaf_s: carbon.ag,
            net_assimilation_umol_co2_m2_leaf_s: carbon.an,
            dark_respiration_umol_co2_m2_leaf_s: carbon.rd,
        });
    }
    if fa * fb > 0.0 {
        return Err(LandSurfaceEnergyError::ConstitutiveDomain("ci_bracket"));
    }
    let mut c = a;
    let mut fc = fa;
    let mut d = b - a;
    let mut mflag = true;
    for _ in 3..=64 {
        let mut s = if fa != fc && fb != fc {
            a * fb * fc / ((fa - fb) * (fa - fc))
                + b * fa * fc / ((fb - fa) * (fb - fc))
                + c * fa * fb / ((fc - fa) * (fc - fb))
        } else {
            b - fb * (b - a) / (fb - fa)
        };
        let left = ((3.0 * a + b) / 4.0).min(b);
        let right = ((3.0 * a + b) / 4.0).max(b);
        if !(left < s && s < right)
            || (mflag && (s - b).abs() >= (b - c).abs() / 2.0)
            || (!mflag && (s - b).abs() >= (c - d).abs() / 2.0)
            || (mflag && (b - c).abs() < 1.0e-6)
            || (!mflag && (c - d).abs() < 1.0e-6)
        {
            s = 0.5 * (a + b);
            mflag = true;
        } else {
            mflag = false;
        }
        let (fs, state_rs) = residual(s)?;
        rs = state_rs;
        d = c;
        c = b;
        fc = fb;
        if fa * fs < 0.0 {
            b = s;
            fb = fs;
        } else {
            a = s;
            fa = fs;
        }
        if fa.abs() < fb.abs() {
            std::mem::swap(&mut a, &mut b);
            std::mem::swap(&mut fa, &mut fb);
        }
        let scale = a.abs().max(b.abs()).max(1.0);
        if fb.abs() <= 1.0e-8 || (b - a).abs() <= 1.0e-6 + 1.0e-10 * scale {
            rs = residual(b)?.1;
            break;
        }
    }
    let carbon = carbon_at_ci(b)?;
    Ok(LeafTrialState {
        surface_q: qsurface,
        rs_s_m: rs,
        ci_pa: b,
        gross_assimilation_umol_co2_m2_leaf_s: carbon.ag,
        net_assimilation_umol_co2_m2_leaf_s: carbon.an,
        dark_respiration_umol_co2_m2_leaf_s: carbon.rd,
    })
}

#[must_use]
fn vulnerability(potential: f64, p50: f64, exponent: f64) -> f64 {
    2.0_f64.powf(-(potential / p50).powf(exponent))
}

struct CoveredOccupancyTrialContext<'a> {
    column: &'a CoveredColumnInputs,
    canopy_air_temperature_k: f64,
    canopy_air_q: f64,
    component_longwave_w_m2: [f64; 4],
    caps: Option<&'a CoveredWaterCaps>,
    frozen: Option<&'a CoveredFrozenBranches>,
    liquid: CoveredLiquidPreparation,
}

fn covered_wet_flux(
    column: &CoveredColumnInputs,
    occupancy: &CoveredOccupancyInputs,
    liquid: CoveredLiquidPreparation,
    wet_surface_temperature_k: f64,
    canopy_air_temperature_k: f64,
    canopy_air_q: f64,
    frozen: Option<&CoveredFrozenBranches>,
) -> Result<(f64, WaterBranch), LandSurfaceEnergyError> {
    let rho = column.pressure_pa / (DRY_AIR_GAS_CONSTANT_J_KG_K * canopy_air_temperature_k);
    let wet_area = liquid.wet_fraction
        * (occupancy.sun.leaf_area_m2_m2_tile
            + occupancy.shade.leaf_area_m2_m2_tile
            + occupancy.stem_area_m2_m2_tile);
    let wet_potential = rho
        * occupancy.gb_wet_m_s
        * (canopy_saturation_q(wet_surface_temperature_k, column.pressure_pa)? - canopy_air_q)
        * wet_area;
    let wet_cap = liquid.preliminary_store / column.interval_s;
    let natural_branch = if wet_potential >= 0.0 && wet_cap <= wet_potential {
        WaterBranch::AuthorizationActiveOrTie
    } else if wet_potential < 0.0 {
        WaterBranch::Condensation
    } else {
        WaterBranch::ConstitutiveLaw
    };
    let branch = frozen
        .and_then(|value| value.wet.get(&occupancy.occupancy_id).copied())
        .unwrap_or(natural_branch);
    Ok((
        if branch == WaterBranch::AuthorizationActiveOrTie {
            wet_cap
        } else {
            wet_potential
        },
        branch,
    ))
}

fn evaluate_covered_occupancy(
    context: &CoveredOccupancyTrialContext<'_>,
    occupancy: &CoveredOccupancyInputs,
    block: &[f64],
) -> Result<CoveredOccupancyEvaluation, LandSurfaceEnergyError> {
    let column = context.column;
    let canopy_air_temperature_k = context.canopy_air_temperature_k;
    let canopy_air_q = context.canopy_air_q;
    let component_longwave_w_m2 = context.component_longwave_w_m2;
    if block.len() != 10 {
        return Err(LandSurfaceEnergyError::topology_domain(
            "covered_occupancy_trial",
        ));
    }
    let (psi_sun, psi_shade, psi_stem, psi_root) = (block[0], block[1], block[2], block[3]);
    let (beta_sun, beta_shade) = (block[4], block[5]);
    let (tsun, tshade, twet, tstem) = (block[6], block[7], block[8], block[9]);
    let rho = column.pressure_pa / (DRY_AIR_GAS_CONSTANT_J_KG_K * canopy_air_temperature_k);
    let wet_fraction = context.liquid.wet_fraction;
    let dry_sun = occupancy.sun.leaf_area_m2_m2_tile * (1.0 - wet_fraction);
    let dry_shade = occupancy.shade.leaf_area_m2_m2_tile * (1.0 - wet_fraction);
    let wet_area = wet_fraction
        * (occupancy.sun.leaf_area_m2_m2_tile
            + occupancy.shade.leaf_area_m2_m2_tile
            + occupancy.stem_area_m2_m2_tile);
    let dry_stem = (1.0 - wet_fraction) * occupancy.stem_area_m2_m2_tile;
    let sun = leaf_trial_state(
        occupancy.sun,
        occupancy.biochemical,
        tsun,
        canopy_air_q,
        beta_sun,
        column,
        occupancy.gb_leaf_m_s,
        occupancy.g0_umol_m2_s,
        occupancy.medlyn_g1_kpa_sqrt,
    )?;
    let shade = leaf_trial_state(
        occupancy.shade,
        occupancy.biochemical,
        tshade,
        canopy_air_q,
        beta_shade,
        column,
        occupancy.gb_leaf_m_s,
        occupancy.g0_umol_m2_s,
        occupancy.medlyn_g1_kpa_sqrt,
    )?;
    // V8 maximum demand is an internal beta=1 evaluation at the current
    // leaf/canopy state. It is never a caller-configurable runtime operand.
    let sun_maximum = leaf_trial_state(
        occupancy.sun,
        occupancy.biochemical,
        tsun,
        canopy_air_q,
        1.0,
        column,
        occupancy.gb_leaf_m_s,
        occupancy.g0_umol_m2_s,
        occupancy.medlyn_g1_kpa_sqrt,
    )?;
    let shade_maximum = leaf_trial_state(
        occupancy.shade,
        occupancy.biochemical,
        tshade,
        canopy_air_q,
        1.0,
        column,
        occupancy.gb_leaf_m_s,
        occupancy.g0_umol_m2_s,
        occupancy.medlyn_g1_kpa_sqrt,
    )?;
    let emax_sun_kg_m2_s = rho * (sun_maximum.surface_q - canopy_air_q)
        / (1.0 / occupancy.gb_leaf_m_s + sun_maximum.rs_s_m)
        * dry_sun;
    let emax_shade_kg_m2_s = rho * (shade_maximum.surface_q - canopy_air_q)
        / (1.0 / occupancy.gb_leaf_m_s + shade_maximum.rs_s_m)
        * dry_shade;
    let sun_e =
        rho * (sun.surface_q - canopy_air_q) / (1.0 / occupancy.gb_leaf_m_s + sun.rs_s_m) * dry_sun;
    let shade_e = rho * (shade.surface_q - canopy_air_q)
        / (1.0 / occupancy.gb_leaf_m_s + shade.rs_s_m)
        * dry_shade;
    let (wet_e, wet_branch) = covered_wet_flux(
        column,
        occupancy,
        context.liquid,
        twet,
        canopy_air_temperature_k,
        canopy_air_q,
        context.frozen,
    )?;
    let q1sun = occupancy.k1_sun_max_s1
        * occupancy.sun.leaf_area_m2_m2_tile
        * vulnerability(
            psi_stem,
            occupancy.p50_xylem_mm,
            occupancy.vulnerability_exponent,
        )
        * (psi_stem - psi_sun);
    let q1shade = occupancy.k1_shade_max_s1
        * occupancy.shade.leaf_area_m2_m2_tile
        * vulnerability(
            psi_stem,
            occupancy.p50_xylem_mm,
            occupancy.vulnerability_exponent,
        )
        * (psi_stem - psi_shade);
    let q2 = occupancy.k2_max / occupancy.height_m
        * vulnerability(
            psi_root,
            occupancy.p50_xylem_mm,
            occupancy.vulnerability_exponent,
        )
        * occupancy.sai
        * (psi_root - psi_stem - 1000.0 * occupancy.height_m);
    let mut root_source_sum = 0.0;
    let mut source_water = Vec::with_capacity(occupancy.root_layers.len());
    for layer in &occupancy.root_layers {
        let law = if layer.accessible && !layer.frozen && layer.root_fraction > 0.0 {
            let kr = occupancy.k3_max_m_s / layer.z3_m
                * vulnerability(
                    layer.soil_potential_mm,
                    occupancy.p50_root_mm,
                    occupancy.vulnerability_exponent,
                );
            let ks = layer.ksoil_m2_s / layer.dxroot_m;
            let series = kr * ks / (kr + ks);
            let rai =
                (occupancy.lai + occupancy.sai) * layer.root_fraction * occupancy.root_to_leaf_area;
            let flux = series * rai * (layer.soil_potential_mm - psi_root + layer.gravity_head_mm);
            if flux < 0.0 {
                return Err(LandSurfaceEnergyError::UnsupportedDomain(
                    "hydraulic_redistribution",
                ));
            }
            flux
        } else {
            0.0
        };
        let key = (occupancy.occupancy_id.clone(), layer.layer_id.clone());
        let supplied = context.caps.and_then(|value| value.root.get(&key));
        let cap_rate = supplied.map(|value| value.authorization_rate_kg_m2_tile_s);
        let natural_branch = if cap_rate.is_some_and(|cap| cap <= law) {
            WaterBranch::AuthorizationActiveOrTie
        } else {
            WaterBranch::ConstitutiveLaw
        };
        let branch = context
            .frozen
            .and_then(|value| value.root.get(&key).copied())
            .unwrap_or(natural_branch);
        let final_flux = if branch == WaterBranch::AuthorizationActiveOrTie {
            cap_rate.ok_or(LandSurfaceEnergyError::water_cardinality(
                "frozen_root_cap_without_authorization",
            ))?
        } else {
            law
        };
        root_source_sum += final_flux;
        let request_rate = supplied.map_or(law.max(0.0), |value| value.request_rate_kg_m2_tile_s);
        let request = request_rate * column.tile_fraction * column.interval_s;
        let authorization =
            cap_rate.map(|amount| amount * column.tile_fraction * column.interval_s);
        let finalized = if branch == WaterBranch::AuthorizationActiveOrTie {
            authorization.ok_or(LandSurfaceEnergyError::water_cardinality(
                "missing_root_authorization",
            ))?
        } else {
            final_flux.max(0.0) * column.tile_fraction * column.interval_s
        };
        source_water.push(SourceWaterFlux {
            occupancy_id: occupancy.occupancy_id.clone(),
            layer_id: layer.layer_id.clone(),
            law_kg_m2_tile_s: law,
            final_kg_m2_tile_s: final_flux,
            request_kg_m2_stand_ground: request,
            authorization_kg_m2_stand_ground: authorization,
            finalized_use_kg_m2_stand_ground: finalized,
            branch,
        });
    }
    let sun_h = rho
        * AIR_HEAT_CAPACITY_J_KG_K
        * occupancy.gb_leaf_m_s
        * dry_sun
        * (tsun - canopy_air_temperature_k);
    let shade_h = rho
        * AIR_HEAT_CAPACITY_J_KG_K
        * occupancy.gb_leaf_m_s
        * dry_shade
        * (tshade - canopy_air_temperature_k);
    let wet_h = rho
        * AIR_HEAT_CAPACITY_J_KG_K
        * occupancy.gb_wet_m_s
        * wet_area
        * (twet - canopy_air_temperature_k);
    let stem_h = rho
        * AIR_HEAT_CAPACITY_J_KG_K
        * occupancy.gb_stem_m_s
        * dry_stem
        * (tstem - canopy_air_temperature_k);
    let component_areas = [dry_sun, dry_shade, wet_area, dry_stem];
    let component_temperatures = [tsun, tshade, twet, tstem];
    let physical_energy_residuals = [
        occupancy.sun.absorbed_shortwave_w_m2_tile * (1.0 - wet_fraction)
            + component_longwave_w_m2[0]
            - sun_h
            - column.latent_heat_j_kg * sun_e,
        occupancy.shade.absorbed_shortwave_w_m2_tile * (1.0 - wet_fraction)
            + component_longwave_w_m2[1]
            - shade_h
            - column.latent_heat_j_kg * shade_e,
        wet_fraction
            * (occupancy.sun.absorbed_shortwave_w_m2_tile
                + occupancy.shade.absorbed_shortwave_w_m2_tile
                + occupancy.stem_absorbed_shortwave_w_m2_tile)
            + component_longwave_w_m2[2]
            - wet_h
            - column.latent_heat_j_kg * wet_e,
        (1.0 - wet_fraction) * occupancy.stem_absorbed_shortwave_w_m2_tile
            + component_longwave_w_m2[3]
            - stem_h,
    ];
    let energy_residuals: [f64; 4] = std::array::from_fn(|index| {
        if component_areas[index].to_bits() == 0.0_f64.to_bits() {
            component_temperatures[index] - canopy_air_temperature_k
        } else {
            physical_energy_residuals[index]
        }
    });
    let residuals = vec![
        sun_e - q1sun,
        shade_e - q1shade,
        sun_e
            - emax_sun_kg_m2_s
                * vulnerability(
                    psi_sun,
                    occupancy.p50_leaf_mm,
                    occupancy.vulnerability_exponent,
                ),
        shade_e
            - emax_shade_kg_m2_s
                * vulnerability(
                    psi_shade,
                    occupancy.p50_leaf_mm,
                    occupancy.vulnerability_exponent,
                ),
        q1sun + q1shade - q2,
        q2 - root_source_sum,
        energy_residuals[0],
        energy_residuals[1],
        energy_residuals[2],
        energy_residuals[3],
    ];
    let water_scale = emax_sun_kg_m2_s
        .max(emax_shade_kg_m2_s)
        .max(q1sun.abs())
        .max(q1shade.abs())
        .max(q2.abs())
        .max(root_source_sum.abs());
    let component_operands = [
        occupancy.sun.absorbed_shortwave_w_m2_tile * (1.0 - wet_fraction),
        occupancy.shade.absorbed_shortwave_w_m2_tile * (1.0 - wet_fraction),
        wet_fraction
            * (occupancy.sun.absorbed_shortwave_w_m2_tile
                + occupancy.shade.absorbed_shortwave_w_m2_tile
                + occupancy.stem_absorbed_shortwave_w_m2_tile),
        (1.0 - wet_fraction) * occupancy.stem_absorbed_shortwave_w_m2_tile,
    ];
    let sensible = [sun_h, shade_h, wet_h, stem_h];
    let latent = [
        column.latent_heat_j_kg * sun_e,
        column.latent_heat_j_kg * shade_e,
        column.latent_heat_j_kg * wet_e,
        0.0,
    ];
    let mut tolerances = vec![crate::physics::water_tolerance(water_scale); 6];
    tolerances.extend((0..4).map(|index| {
        if component_areas[index].to_bits() == 0.0_f64.to_bits() {
            crate::physics::energy_tolerance(1.0)
        } else {
            crate::physics::energy_tolerance(
                component_operands[index].abs()
                    + component_longwave_w_m2[index].abs()
                    + sensible[index].abs()
                    + latent[index].abs(),
            )
        }
    }));
    let liquid = finalize_covered_liquid(
        context.liquid,
        wet_e * column.interval_s,
        twet,
        if context.caps.is_some() {
            CoveredLiquidPass::FixedAuthorizationFinal
        } else {
            CoveredLiquidPass::Potential
        },
    )?;
    Ok(CoveredOccupancyEvaluation {
        residuals,
        tolerances,
        source_water,
        canopy_sensible_w_m2: sun_h + shade_h + wet_h + stem_h,
        canopy_vapor_kg_m2_s: sun_e + shade_e + wet_e,
        wet_vapor_kg_m2_s: wet_e,
        wet_branch,
        component_temperatures_k: [tsun, tshade, twet, tstem],
        ci_pa: [sun.ci_pa, shade.ci_pa],
        gross_assimilation_umol_co2_m2_leaf_s: [
            sun.gross_assimilation_umol_co2_m2_leaf_s,
            shade.gross_assimilation_umol_co2_m2_leaf_s,
        ],
        net_assimilation_umol_co2_m2_leaf_s: [
            sun.net_assimilation_umol_co2_m2_leaf_s,
            shade.net_assimilation_umol_co2_m2_leaf_s,
        ],
        dark_respiration_umol_co2_m2_leaf_s: [
            sun.dark_respiration_umol_co2_m2_leaf_s,
            shade.dark_respiration_umol_co2_m2_leaf_s,
        ],
        emax_kg_m2_s: [emax_sun_kg_m2_s, emax_shade_kg_m2_s],
        liquid,
        absorbed_shortwave_w_m2: component_operands,
        net_longwave_w_m2: component_longwave_w_m2,
        sensible_to_canopy_air_w_m2: sensible,
        signed_vapor_to_canopy_air_kg_m2_s: [sun_e, shade_e, wet_e, 0.0],
    })
}

pub fn evaluate_covered_occupancy_block(
    column: &CoveredColumnInputs,
    occupancy: &CoveredOccupancyInputs,
    block: &[f64],
    canopy_air_temperature_k: f64,
    canopy_air_q: f64,
    component_longwave_w_m2: [f64; 4],
) -> Result<Vec<f64>, LandSurfaceEnergyError> {
    let liquid = prepare_covered_liquid(occupancy, column.top_rain_kg_m2_tile)?;
    let context = CoveredOccupancyTrialContext {
        column,
        canopy_air_temperature_k,
        canopy_air_q,
        component_longwave_w_m2,
        caps: None,
        frozen: None,
        liquid,
    };
    Ok(evaluate_covered_occupancy(&context, occupancy, block)?.residuals)
}

fn validate_covered_caps(
    column: &CoveredColumnInputs,
    caps: Option<&CoveredWaterCaps>,
) -> Result<(), LandSurfaceEnergyError> {
    let Some(caps) = caps else {
        return Ok(());
    };
    let expected: BTreeSet<(String, String)> = column
        .occupancies
        .iter()
        .flat_map(|occupancy| {
            occupancy
                .root_layers
                .iter()
                .map(|layer| (occupancy.occupancy_id.clone(), layer.layer_id.clone()))
        })
        .collect();
    let actual: BTreeSet<_> = caps.root.keys().cloned().collect();
    if expected != actual {
        return Err(LandSurfaceEnergyError::water_identity(
            "covered_root_authorization_identity",
        ));
    }
    if caps
        .root
        .values()
        .chain(std::iter::once(&caps.ground))
        .any(|value| {
            !value.request_rate_kg_m2_tile_s.is_finite()
                || !value.authorization_rate_kg_m2_tile_s.is_finite()
        })
    {
        return Err(LandSurfaceEnergyError::water_domain(
            "covered_authorization_domain",
        ));
    }
    if caps
        .root
        .values()
        .chain(std::iter::once(&caps.ground))
        .any(|value| {
            value.authorization_rate_kg_m2_tile_s < 0.0
                || value.request_rate_kg_m2_tile_s < value.authorization_rate_kg_m2_tile_s
        })
    {
        return Err(LandSurfaceEnergyError::water_bound(
            "covered_authorization_domain",
        ));
    }
    Ok(())
}

fn covered_trial_is_valid(trial: &[f64], occupancy_count: usize) -> bool {
    if trial.len() < 10 * occupancy_count + 4 || trial.iter().any(|value| !value.is_finite()) {
        return false;
    }
    for index in 0..occupancy_count {
        let block = &trial[index * 10..(index + 1) * 10];
        if !(0.0..=1.0).contains(&block[4])
            || !(0.0..=1.0).contains(&block[5])
            || block[6..10]
                .iter()
                .any(|value| !(200.0..=350.0).contains(value))
        {
            return false;
        }
    }
    let common = &trial[10 * occupancy_count..];
    (200.0..=350.0).contains(&common[0])
        && (0.0..=0.1).contains(&common[1])
        && common[2..]
            .iter()
            .all(|value| (200.0..=350.0).contains(value))
}

fn validate_covered_shortwave_inputs(
    column: &CoveredColumnInputs,
) -> Result<(), LandSurfaceEnergyError> {
    column.shortwave.incident_w_m2_tile.validate_nonnegative()?;
    column
        .shortwave
        .top_reflected_w_m2_tile
        .validate_nonnegative()?;
    column
        .shortwave
        .ground_absorbed_by_incident_w_m2_tile
        .validate_nonnegative()?;
    column
        .ground
        .terminal_shortwave_w_m2_tile
        .validate_nonnegative()?;
    if column.shortwave.occupancies.len() != column.occupancies.len() {
        return Err(LandSurfaceEnergyError::topology_cardinality(
            "covered shortwave occupancy set",
        ));
    }
    for (radiation, occupancy) in column.shortwave.occupancies.iter().zip(&column.occupancies) {
        radiation
            .sun_leaf_absorbed_w_m2_tile
            .validate_nonnegative()?;
        radiation
            .shade_leaf_absorbed_w_m2_tile
            .validate_nonnegative()?;
        radiation.stem_absorbed_w_m2_tile.validate_nonnegative()?;
        if radiation.occupancy_id != occupancy.occupancy_id
            || radiation.sun_leaf_absorbed_w_m2_tile.total().to_bits()
                != occupancy.sun.absorbed_shortwave_w_m2_tile.to_bits()
            || radiation.shade_leaf_absorbed_w_m2_tile.total().to_bits()
                != occupancy.shade.absorbed_shortwave_w_m2_tile.to_bits()
            || radiation.stem_absorbed_w_m2_tile.total().to_bits()
                != occupancy.stem_absorbed_shortwave_w_m2_tile.to_bits()
        {
            return Err(LandSurfaceEnergyError::ComponentClosure(
                "covered bound shortwave component identity",
            ));
        }
    }
    let incident = directional_values(column.shortwave.incident_w_m2_tile);
    let reflected = directional_values(column.shortwave.top_reflected_w_m2_tile);
    let ground_absorbed =
        directional_values(column.shortwave.ground_absorbed_by_incident_w_m2_tile);
    for direction in 0..4 {
        let canopy_absorbed: f64 = column
            .shortwave
            .occupancies
            .iter()
            .map(|occupancy| {
                directional_values(occupancy.sun_leaf_absorbed_w_m2_tile)[direction]
                    + directional_values(occupancy.shade_leaf_absorbed_w_m2_tile)[direction]
                    + directional_values(occupancy.stem_absorbed_w_m2_tile)[direction]
            })
            .sum();
        let residual = incident[direction]
            - reflected[direction]
            - canopy_absorbed
            - ground_absorbed[direction];
        if residual.abs()
            > energy_tolerance(
                incident[direction].abs()
                    + reflected[direction].abs()
                    + canopy_absorbed.abs()
                    + ground_absorbed[direction].abs(),
            )
        {
            return Err(LandSurfaceEnergyError::ComponentClosure(
                "covered whole-column band/direction shortwave",
            ));
        }
    }
    Ok(())
}

fn directional_values(value: BandDirectionalFluxes) -> [f64; 4] {
    [
        value.direct_vis,
        value.diffuse_vis,
        value.direct_nir,
        value.diffuse_nir,
    ]
}

pub fn evaluate_covered_column(
    column: &CoveredColumnInputs,
    trial: &[f64],
    caps: Option<&CoveredWaterCaps>,
    frozen: Option<&CoveredFrozenBranches>,
) -> Result<CoveredColumnEvaluation, LandSurfaceEnergyError> {
    if column.occupancies.is_empty() || column.ground.soil_nodes.is_empty() {
        return Err(LandSurfaceEnergyError::topology_cardinality(
            "covered_column",
        ));
    }
    validate_covered_caps(column, caps)?;
    validate_covered_shortwave_inputs(column)?;
    let expected = 10 * column.occupancies.len() + 3 + column.ground.soil_nodes.len();
    if trial.len() != expected || !covered_trial_is_valid(trial, column.occupancies.len()) {
        return Err(LandSurfaceEnergyError::ConstitutiveDomain(
            "covered_trial_shape_or_bounds",
        ));
    }
    let common_offset = 10 * column.occupancies.len();
    let canopy_temperature = trial[common_offset];
    let canopy_q = trial[common_offset + 1];
    let ground_temperature = trial[common_offset + 2];
    let soil_temperatures = &trial[common_offset + 3..];
    if !column.top_rain_kg_m2_tile.is_finite() || column.top_rain_kg_m2_tile < 0.0 {
        return Err(LandSurfaceEnergyError::ConstitutiveDomain(
            "covered top rain",
        ));
    }
    let mut incident_rain = column.top_rain_kg_m2_tile;
    let mut ground_stemflow = 0.0;
    let mut liquid_preparations = Vec::with_capacity(column.occupancies.len());
    let mut routed_liquid = Vec::with_capacity(column.occupancies.len());
    for (index, occupancy) in column.occupancies.iter().enumerate() {
        let block = &trial[index * 10..(index + 1) * 10];
        let preparation = prepare_covered_liquid(occupancy, incident_rain)?;
        let (wet_flux, _) = covered_wet_flux(
            column,
            occupancy,
            preparation,
            block[8],
            canopy_temperature,
            canopy_q,
            frozen,
        )?;
        let liquid = finalize_covered_liquid(
            preparation,
            wet_flux * column.interval_s,
            block[8],
            if caps.is_some() {
                CoveredLiquidPass::FixedAuthorizationFinal
            } else {
                CoveredLiquidPass::Potential
            },
        )?;
        incident_rain = liquid.throughfall_kg_m2_tile
            + liquid.initial_drainage_kg_m2_tile
            + liquid.second_drainage_kg_m2_tile;
        ground_stemflow += liquid.stemflow_kg_m2_tile;
        liquid_preparations.push(preparation);
        routed_liquid.push(liquid);
    }
    let longwave_layers: Vec<_> = column
        .occupancies
        .iter()
        .zip(&liquid_preparations)
        .enumerate()
        .map(|(index, (occupancy, liquid))| {
            let block = &trial[index * 10..(index + 1) * 10];
            let dry_sun = occupancy.sun.leaf_area_m2_m2_tile * (1.0 - liquid.wet_fraction);
            let dry_shade = occupancy.shade.leaf_area_m2_m2_tile * (1.0 - liquid.wet_fraction);
            let wet = liquid.wet_fraction
                * (occupancy.sun.leaf_area_m2_m2_tile
                    + occupancy.shade.leaf_area_m2_m2_tile
                    + occupancy.stem_area_m2_m2_tile);
            crate::physics::CanopyLongwaveLayer {
                clumping_index: occupancy.clumping_index,
                leaf_area_index: occupancy.lai,
                stem_area_index: occupancy.sai,
                component_areas: [
                    dry_sun,
                    dry_shade,
                    wet,
                    (1.0 - liquid.wet_fraction) * occupancy.stem_area_m2_m2_tile,
                ],
                component_temperatures_k: [block[6], block[7], block[8], block[9]],
            }
        })
        .collect();
    let longwave = crate::physics::reciprocal_longwave_column(
        column.atmospheric_downward_longwave_w_m2,
        ground_temperature,
        &longwave_layers,
    )?;
    let mut occupancy_results = Vec::with_capacity(column.occupancies.len());
    let mut raw = Vec::new();
    let mut tolerances = Vec::new();
    for (index, occupancy) in column.occupancies.iter().enumerate() {
        let context = CoveredOccupancyTrialContext {
            column,
            canopy_air_temperature_k: canopy_temperature,
            canopy_air_q: canopy_q,
            component_longwave_w_m2: longwave.component_net_w_m2[index],
            caps,
            frozen,
            liquid: liquid_preparations[index],
        };
        let result =
            evaluate_covered_occupancy(&context, occupancy, &trial[index * 10..(index + 1) * 10])?;
        if result.liquid != routed_liquid[index] {
            return Err(LandSurfaceEnergyError::OwnerEnvelope(
                "covered E04 routed/final evaluation mismatch",
            ));
        }
        raw.extend(result.residuals.iter().copied());
        tolerances.extend(result.tolerances.iter().copied());
        occupancy_results.push(result);
    }
    let rho = column.pressure_pa / (DRY_AIR_GAS_CONSTANT_J_KG_K * canopy_temperature);
    let ground_resistance = crate::physics::under_canopy_neutral_resistance(
        column.under_canopy_geometry,
        column.reference_wind_m_s,
    )?;
    let ground = &column.ground;
    let (ground_law, _) = if ground.class == SurfaceClassKind::BareMineralSoil
        && ground.surface_liquid_kg_m2_tile == 0.0
    {
        let parameters = ground
            .bare_soil
            .ok_or(LandSurfaceEnergyError::ConstitutiveDomain(
                "missing_covered_bare_soil_parameters",
            ))?;
        let detail = bare_soil_vapor(BareSoilVaporOperands {
            top_layer_liquid_kg_m2: parameters.top_layer_liquid_kg_m2,
            top_layer_ice_kg_m2: parameters.top_layer_ice_kg_m2,
            top_layer_depth_m: ground.soil_nodes[0].depth_m,
            porosity: parameters.porosity,
            saturated_matric_potential_mm: parameters.saturated_matric_potential_mm,
            clapp_hornberger_b: parameters.clapp_hornberger_b,
            theta_initial: parameters.theta_initial,
            surface_temperature_k: ground_temperature,
            recipient_specific_humidity_kg_kg: canopy_q,
            pressure_pa: column.pressure_pa,
            aerodynamic_vapor_resistance_s_m: ground_resistance.resistance_s_m,
            moist_air_density_kg_m3: rho,
        })?;
        (detail.signed_flux_kg_m2_s, Some(detail))
    } else {
        let humidity = match ground.class {
            SurfaceClassKind::BareMineralSoil => 1.0,
            SurfaceClassKind::ForestLitter => litter_relative_humidity(
                ground.surface_liquid_kg_m2_tile,
                ground.litter_capacity_kg_m2_tile.ok_or(
                    LandSurfaceEnergyError::ConstitutiveDomain("covered_litter_capacity"),
                )?,
            )?,
        };
        let saturated = canopy_saturation_q(ground_temperature, column.pressure_pa)?;
        let surface_q = humidity * saturated + (1.0 - humidity) * canopy_q;
        (
            rho * (surface_q - canopy_q) / ground_resistance.resistance_s_m,
            None,
        )
    };
    let natural_ground_branch = if ground_law < 0.0 {
        WaterBranch::Condensation
    } else if caps.is_some_and(|value| value.ground.authorization_rate_kg_m2_tile_s <= ground_law) {
        WaterBranch::AuthorizationActiveOrTie
    } else {
        WaterBranch::ConstitutiveLaw
    };
    let ground_branch = frozen
        .and_then(|value| value.ground)
        .unwrap_or(natural_ground_branch);
    let final_ground_vapor = if ground_branch == WaterBranch::AuthorizationActiveOrTie {
        caps.ok_or(LandSurfaceEnergyError::water_cardinality(
            "frozen_ground_cap_without_authorization",
        ))?
        .ground
        .authorization_rate_kg_m2_tile_s
    } else {
        ground_law
    };
    let uses_store = !(ground.class == SurfaceClassKind::BareMineralSoil
        && ground.surface_liquid_kg_m2_tile == 0.0);
    let ending_water = if uses_store {
        ground.surface_liquid_kg_m2_tile - final_ground_vapor.max(0.0) * column.interval_s
            + (-final_ground_vapor).max(0.0) * column.interval_s
    } else {
        ground.surface_liquid_kg_m2_tile
    };
    if ending_water < -1.0e-14 {
        return Err(LandSurfaceEnergyError::water_bound(
            "covered_surface_water_negative",
        ));
    }
    let ground_sensible =
        rho * AIR_HEAT_CAPACITY_J_KG_K * (ground_temperature - canopy_temperature)
            / ground_resistance.resistance_s_m;
    let reference_heat =
        rho * AIR_HEAT_CAPACITY_J_KG_K * (canopy_temperature - column.air_temperature_k)
            / column.canopy_to_atmosphere_heat_resistance_s_m;
    let reference_vapor = rho * (canopy_q - column.air_specific_humidity_kg_kg)
        / column.canopy_to_atmosphere_vapor_resistance_s_m;
    let canopy_sensible: f64 = occupancy_results
        .iter()
        .map(|value| value.canopy_sensible_w_m2)
        .sum();
    let canopy_vapor: f64 = occupancy_results
        .iter()
        .map(|value| value.canopy_vapor_kg_m2_s)
        .sum();
    let shared_heat = canopy_sensible + ground_sensible - reference_heat;
    let shared_vapor = canopy_vapor + final_ground_vapor - reference_vapor;
    let shared_heat_scale =
        (canopy_sensible.abs() + ground_sensible.abs() + reference_heat.abs()).max(1.0);
    let shared_vapor_scale = canopy_vapor
        .abs()
        .max(final_ground_vapor.abs())
        .max(reference_vapor.abs());
    raw.extend([shared_heat, shared_vapor]);
    tolerances.extend([
        crate::physics::energy_tolerance(shared_heat_scale),
        crate::physics::water_tolerance(shared_vapor_scale),
    ]);
    let shortwave = partition_ground_shortwave(
        ground.terminal_shortwave_w_m2_tile,
        ground.surface_vis_albedo,
        ground.surface_nir_albedo,
    )?;
    let ground_vapor_energy = vapor_export_w_m2(final_ground_vapor, ground_temperature)?;
    let (ground_storage, ending_enthalpy, beginning_ground_temperature) = match ground
        .storage_branch
    {
        SurfaceStorageBranch::FiniteCapacity => {
            let ending_capacity = ground.surface_dry_heat_capacity_j_m2_k
                + ending_water.max(0.0) * WATER_HEAT_CAPACITY_J_KG_K;
            let ending = ending_capacity * (ground_temperature - REFERENCE_TEMPERATURE_K);
            let beginning_capacity = ground.surface_dry_heat_capacity_j_m2_k
                + ground.surface_liquid_kg_m2_tile * WATER_HEAT_CAPACITY_J_KG_K;
            (
                (ending - ground.surface_enthalpy_j_m2_tile) / column.interval_s,
                ending,
                REFERENCE_TEMPERATURE_K + ground.surface_enthalpy_j_m2_tile / beginning_capacity,
            )
        }
        SurfaceStorageBranch::EquilibriumZero => (0.0, 0.0, ground_temperature),
    };
    let first = &ground.soil_nodes[0];
    let surface_conductance = harmonic_interface_conductance_w_m2_k(
        ground.surface_depth_m,
        ground.surface_conductivity_w_m_k,
        first.depth_m,
        first.conductivity_w_m_k,
    )?;
    let mut begin_fluxes =
        vec![surface_conductance * (beginning_ground_temperature - first.beginning_temperature_k)];
    let mut end_fluxes = vec![surface_conductance * (ground_temperature - soil_temperatures[0])];
    for index in 0..ground.soil_nodes.len().saturating_sub(1) {
        let upper = &ground.soil_nodes[index];
        let lower = &ground.soil_nodes[index + 1];
        let conductance = harmonic_interface_conductance_w_m2_k(
            upper.depth_m,
            upper.conductivity_w_m_k,
            lower.depth_m,
            lower.conductivity_w_m_k,
        )?;
        begin_fluxes
            .push(conductance * (upper.beginning_temperature_k - lower.beginning_temperature_k));
        end_fluxes.push(conductance * (soil_temperatures[index] - soil_temperatures[index + 1]));
    }
    let ground_heat: Vec<f64> = begin_fluxes
        .iter()
        .zip(end_fluxes.iter())
        .map(|(begin, end)| 0.5 * (begin + end))
        .collect();
    let surface_operands = [
        shortwave.absorbed.total(),
        longwave.ground_net_w_m2,
        -ground_sensible,
        -ground_vapor_energy,
        -ground_heat[0],
        -ground_storage,
    ];
    let surface_residual: f64 = surface_operands.iter().sum();
    raw.push(surface_residual);
    tolerances.push(crate::physics::energy_tolerance(
        surface_operands.iter().map(|value| value.abs()).sum(),
    ));
    for (index, node) in ground.soil_nodes.iter().enumerate() {
        let incoming = ground_heat[index];
        let outgoing = ground_heat.get(index + 1).copied().unwrap_or(0.0);
        let storage = node.heat_capacity_j_m2_k
            * (soil_temperatures[index] - node.beginning_temperature_k)
            / column.interval_s;
        raw.push(incoming - outgoing - storage);
        tolerances.push(crate::physics::energy_tolerance(
            incoming.abs() + outgoing.abs() + storage.abs(),
        ));
    }
    let normalized_residuals = raw
        .iter()
        .zip(tolerances.iter())
        .map(|(residual, tolerance)| residual / tolerance)
        .collect();
    let ground_authorization = caps.map(|value| {
        value.ground.authorization_rate_kg_m2_tile_s * column.tile_fraction * column.interval_s
    });
    let ground_finalized = if ground_branch == WaterBranch::AuthorizationActiveOrTie {
        ground_authorization.ok_or(LandSurfaceEnergyError::water_cardinality(
            "missing_ground_authorization",
        ))?
    } else {
        final_ground_vapor.max(0.0) * column.tile_fraction * column.interval_s
    };
    Ok(CoveredColumnEvaluation {
        raw_residuals: raw,
        normalized_residuals,
        tolerances,
        occupancies: occupancy_results,
        canopy_air_temperature_k: canopy_temperature,
        canopy_air_specific_humidity_kg_kg: canopy_q,
        ground_temperature_k: ground_temperature,
        soil_temperature_k: soil_temperatures.to_vec(),
        ground_water: GroundWaterFlux {
            law_kg_m2_tile_s: ground_law,
            final_kg_m2_tile_s: final_ground_vapor,
            request_kg_m2_stand_ground: caps.map_or(ground_law.max(0.0), |value| {
                value.ground.request_rate_kg_m2_tile_s
            }) * column.tile_fraction
                * column.interval_s,
            authorization_kg_m2_stand_ground: ground_authorization,
            finalized_use_kg_m2_stand_ground: ground_finalized,
            condensation_credit_kg_m2_stand_ground: (-final_ground_vapor).max(0.0)
                * column.tile_fraction
                * column.interval_s,
            branch: ground_branch,
        },
        ground_heat_cn_w_m2_tile: ground_heat,
        ground_storage_w_m2_tile: ground_storage,
        ending_surface_enthalpy_j_m2_tile: ending_enthalpy,
        whole_column_longwave: longwave,
        ground_canopy_release_kg_m2_tile: incident_rain,
        ground_stemflow_kg_m2_tile: ground_stemflow,
        ground_sensible_to_canopy_air_w_m2: ground_sensible,
        sensible_to_reference_air_w_m2: reference_heat,
        vapor_to_reference_air_kg_m2_s: reference_vapor,
    })
}

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CoveredStepNorms {
    pub hydraulic_mm: f64,
    pub beta: f64,
    pub temperature_k: f64,
    pub humidity_kg_kg: f64,
    pub ci_pa: f64,
}

impl CoveredStepNorms {
    fn accepted(self) -> bool {
        self.hydraulic_mm <= 1.0e-7
            && self.beta <= 1.0e-10
            && self.temperature_k <= 1.0e-8
            && self.humidity_kg_kg <= 1.0e-12
    }

    fn diagnostics(self) -> StepNorms {
        StepNorms {
            temperature_k: Some(self.temperature_k),
            humidity_kg_kg: Some(self.humidity_kg_kg),
            ci_pa: Some(self.ci_pa),
            hydraulic_mm: Some(self.hydraulic_mm),
            beta: Some(self.beta),
        }
    }
}

fn empty_step_norms() -> StepNorms {
    StepNorms {
        temperature_k: None,
        humidity_kg_kg: None,
        ci_pa: None,
        hydraulic_mm: None,
        beta: None,
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct CoveredColumnCandidate {
    pub solution: Vec<f64>,
    pub evaluation: CoveredColumnEvaluation,
    pub surface_enthalpy_j_m2_tile: f64,
    pub soil_temperature_k: Vec<f64>,
    pub root_water: Vec<SourceWaterFlux>,
    pub ground_water: GroundWaterFlux,
    pub iterations: u32,
    pub backtracking_count: u32,
    pub step_norms: CoveredStepNorms,
}

#[derive(Clone, Debug, PartialEq)]
pub enum CoveredColumnSolveOutcome {
    Accepted(Box<CoveredColumnCandidate>),
    Rejected(NumericalFailure),
}

pub(crate) fn covered_failure_residuals(
    beginning: &CoveredColumnInputs,
    detail: &CoveredColumnEvaluation,
) -> Vec<NormalizedResidual> {
    const OCCUPANCY_IDENTITIES: [&str; 10] = [
        "sun_gas_minus_q1",
        "shade_gas_minus_q1",
        "sun_gas_minus_vulnerability_demand",
        "shade_gas_minus_vulnerability_demand",
        "q1_sum_minus_q2",
        "q2_minus_root_source_sum",
        "sun_leaf_energy",
        "shade_leaf_energy",
        "wet_surface_energy",
        "dry_stem_energy",
    ];
    let occupancy_count = beginning.occupancies.len();
    detail
        .raw_residuals
        .iter()
        .zip(&detail.tolerances)
        .zip(&detail.normalized_residuals)
        .enumerate()
        .map(|(index, ((raw, tolerance), normalized))| {
            let (identity, unit) = if index < 10 * occupancy_count {
                let occupancy = index / 10;
                let local = index % 10;
                let prefix = if occupancy_count == 1 {
                    String::new()
                } else {
                    format!("{}:", beginning.occupancies[occupancy].occupancy_id)
                };
                (
                    format!("{prefix}{}", OCCUPANCY_IDENTITIES[local]),
                    if local < 6 {
                        ResidualUnit::KilogramsPerSquareMeterSecond
                    } else {
                        ResidualUnit::WattsPerSquareMeter
                    },
                )
            } else {
                let shared = index - 10 * occupancy_count;
                match shared {
                    0 => (
                        "shared_canopy_air_heat".into(),
                        ResidualUnit::WattsPerSquareMeter,
                    ),
                    1 => (
                        "shared_canopy_air_vapor".into(),
                        ResidualUnit::KilogramsPerSquareMeterSecond,
                    ),
                    2 => (
                        "ground_surface_energy".into(),
                        ResidualUnit::WattsPerSquareMeter,
                    ),
                    soil => (
                        format!(
                            "soil_thermal:{}",
                            beginning.ground.soil_nodes[soil - 3].layer_id
                        ),
                        ResidualUnit::WattsPerSquareMeter,
                    ),
                }
            };
            diagnostic_residual(identity, *raw, *tolerance, *normalized, unit)
        })
        .collect()
}

fn freeze_covered_branches(detail: &CoveredColumnEvaluation) -> CoveredFrozenBranches {
    let mut frozen = CoveredFrozenBranches {
        ground: Some(detail.ground_water.branch),
        ..Default::default()
    };
    for occupancy in &detail.occupancies {
        if let Some(identity) = occupancy
            .source_water
            .first()
            .map(|value| value.occupancy_id.clone())
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

fn covered_step_norms(
    applied: &[f64],
    occupancy_count: usize,
    before: &CoveredColumnEvaluation,
    after: &CoveredColumnEvaluation,
) -> CoveredStepNorms {
    let mut result = CoveredStepNorms::default();
    for index in 0..occupancy_count {
        let offset = 10 * index;
        result.hydraulic_mm = result.hydraulic_mm.max(
            applied[offset..offset + 4]
                .iter()
                .map(|value| value.abs())
                .fold(0.0, f64::max),
        );
        result.beta = result.beta.max(
            applied[offset + 4..offset + 6]
                .iter()
                .map(|value| value.abs())
                .fold(0.0, f64::max),
        );
        result.temperature_k = result.temperature_k.max(
            applied[offset + 6..offset + 10]
                .iter()
                .map(|value| value.abs())
                .fold(0.0, f64::max),
        );
        result.ci_pa = result
            .ci_pa
            .max((after.occupancies[index].ci_pa[0] - before.occupancies[index].ci_pa[0]).abs())
            .max((after.occupancies[index].ci_pa[1] - before.occupancies[index].ci_pa[1]).abs());
    }
    let common = 10 * occupancy_count;
    result.temperature_k = result.temperature_k.max(applied[common].abs()).max(
        applied[common + 2..]
            .iter()
            .map(|value| value.abs())
            .fold(0.0, f64::max),
    );
    result.humidity_kg_kg = applied[common + 1].abs();
    result
}

pub fn solve_covered_column(
    beginning: &CoveredColumnInputs,
    caps: Option<&CoveredWaterCaps>,
    initial_trial: Vec<f64>,
) -> Result<CoveredColumnSolveOutcome, LandSurfaceEnergyError> {
    validate_covered_caps(beginning, caps)?;
    if !covered_trial_is_valid(&initial_trial, beginning.occupancies.len()) {
        return Err(LandSurfaceEnergyError::ConstitutiveDomain(
            "covered_initial_trial",
        ));
    }
    let mut x = initial_trial;
    let mut last_steps = None;
    let mut backtracking_count = 0;
    let mut pivot = None;
    let mut matrix_norm = None;
    for iteration in 0..=MAX_NEWTON_ITERATIONS {
        let detail = evaluate_covered_column(beginning, &x, caps, None)?;
        let norm = normalized_infinity_norm(&detail.normalized_residuals);
        if norm <= 1.0 && last_steps.is_some_and(CoveredStepNorms::accepted) {
            let root_water = detail
                .occupancies
                .iter()
                .flat_map(|value| value.source_water.clone())
                .collect();
            return Ok(CoveredColumnSolveOutcome::Accepted(Box::new(
                CoveredColumnCandidate {
                    solution: x,
                    surface_enthalpy_j_m2_tile: detail.ending_surface_enthalpy_j_m2_tile,
                    soil_temperature_k: detail.soil_temperature_k.clone(),
                    root_water,
                    ground_water: detail.ground_water,
                    iterations: iteration,
                    backtracking_count,
                    step_norms: last_steps.unwrap_or_default(),
                    evaluation: detail,
                },
            )));
        }
        if iteration == MAX_NEWTON_ITERATIONS {
            let (occupancy_id, active_bounds) = covered_failure_metadata(beginning, &detail, &x);
            return Ok(CoveredColumnSolveOutcome::Rejected(NumericalFailure {
                kind: NumericalFailureKind::IterationLimit,
                iterations: iteration,
                ordered_residuals: covered_failure_residuals(beginning, &detail),
                normalized_residuals: detail.normalized_residuals.clone(),
                occupancy_id,
                active_bounds,
                failed_solution: x,
                backtracking_count,
                step_norms: last_steps.map_or_else(empty_step_norms, CoveredStepNorms::diagnostics),
                pivot_magnitude: pivot,
                matrix_norm,
            }));
        }
        let frozen = freeze_covered_branches(&detail);
        let units: Vec<f64> = (0..beginning.occupancies.len())
            .flat_map(|_| [1000.0, 1000.0, 1000.0, 1000.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0])
            .chain([1.0, 0.001, 1.0])
            .chain(std::iter::repeat_n(1.0, beginning.ground.soil_nodes.len()))
            .collect();
        let perturbations: Vec<f64> = x
            .iter()
            .zip(units.iter())
            .map(|(value, unit)| f64::EPSILON.sqrt() * value.abs().max(*unit))
            .collect();
        let mut jacobian = vec![vec![0.0; x.len()]; x.len()];
        for column_index in 0..x.len() {
            let mut minus = x.clone();
            let mut plus = x.clone();
            minus[column_index] -= perturbations[column_index];
            plus[column_index] += perturbations[column_index];
            let minus_detail = evaluate_covered_column(beginning, &minus, caps, Some(&frozen))?;
            let plus_detail = evaluate_covered_column(beginning, &plus, caps, Some(&frozen))?;
            for row in 0..x.len() {
                jacobian[row][column_index] = (plus_detail.normalized_residuals[row]
                    - minus_detail.normalized_residuals[row])
                    / (2.0 * perturbations[column_index]);
            }
        }
        let rhs: Vec<f64> = detail
            .normalized_residuals
            .iter()
            .map(|value| -value)
            .collect();
        let (delta, current_pivot, current_matrix_norm) = match solve_linear(&jacobian, &rhs) {
            Ok(value) => value,
            Err(evidence) => {
                let (occupancy_id, active_bounds) =
                    covered_failure_metadata(beginning, &detail, &x);
                return Ok(CoveredColumnSolveOutcome::Rejected(NumericalFailure {
                    kind: NumericalFailureKind::SingularPivot,
                    iterations: iteration,
                    ordered_residuals: covered_failure_residuals(beginning, &detail),
                    normalized_residuals: detail.normalized_residuals.clone(),
                    occupancy_id,
                    active_bounds,
                    failed_solution: x,
                    backtracking_count,
                    step_norms: last_steps
                        .map_or_else(empty_step_norms, CoveredStepNorms::diagnostics),
                    pivot_magnitude: Some(evidence.pivot),
                    matrix_norm: Some(evidence.matrix_norm),
                }));
            }
        };
        pivot = Some(current_pivot);
        matrix_norm = Some(current_matrix_norm);
        let prospective: Vec<f64> = x
            .iter()
            .zip(delta.iter())
            .map(|(value, change)| value + change)
            .collect();
        if norm <= 1.0 && covered_trial_is_valid(&prospective, beginning.occupancies.len()) {
            let prospective_detail = evaluate_covered_column(beginning, &prospective, caps, None)?;
            let prospective_steps = covered_step_norms(
                &delta,
                beginning.occupancies.len(),
                &detail,
                &prospective_detail,
            );
            if prospective_steps.accepted() {
                let root_water = detail
                    .occupancies
                    .iter()
                    .flat_map(|value| value.source_water.clone())
                    .collect();
                return Ok(CoveredColumnSolveOutcome::Accepted(Box::new(
                    CoveredColumnCandidate {
                        solution: x,
                        surface_enthalpy_j_m2_tile: detail.ending_surface_enthalpy_j_m2_tile,
                        soil_temperature_k: detail.soil_temperature_k.clone(),
                        root_water,
                        ground_water: detail.ground_water,
                        iterations: iteration,
                        backtracking_count,
                        step_norms: prospective_steps,
                        evaluation: detail,
                    },
                )));
            }
        }
        let mut accepted = None;
        let mut rejected_step_norms = None;
        for exponent in 0..=MAX_BACKTRACKING_HALVINGS {
            let factor = 0.5_f64.powf(f64::from(exponent));
            let trial: Vec<f64> = x
                .iter()
                .zip(delta.iter())
                .map(|(value, change)| value + factor * change)
                .collect();
            if !covered_trial_is_valid(&trial, beginning.occupancies.len()) {
                continue;
            }
            let Ok(trial_detail) = evaluate_covered_column(beginning, &trial, caps, None) else {
                continue;
            };
            let applied: Vec<f64> = delta.iter().map(|value| factor * value).collect();
            let steps = covered_step_norms(
                &applied,
                beginning.occupancies.len(),
                &detail,
                &trial_detail,
            );
            rejected_step_norms = Some(steps);
            if is_strict_residual_decrease(norm, &trial_detail.normalized_residuals) {
                accepted = Some((trial, steps, exponent));
                break;
            }
        }
        if let Some((trial, steps, exponent)) = accepted {
            x = trial;
            last_steps = Some(steps);
            backtracking_count += exponent;
        } else {
            let (occupancy_id, active_bounds) = covered_failure_metadata(beginning, &detail, &x);
            return Ok(CoveredColumnSolveOutcome::Rejected(NumericalFailure {
                kind: NumericalFailureKind::BacktrackingLimit,
                iterations: iteration,
                ordered_residuals: covered_failure_residuals(beginning, &detail),
                normalized_residuals: detail.normalized_residuals.clone(),
                occupancy_id,
                active_bounds,
                failed_solution: x,
                backtracking_count: backtracking_count + MAX_BACKTRACKING_HALVINGS,
                step_norms: rejected_step_norms
                    .or(last_steps)
                    .map_or_else(empty_step_norms, CoveredStepNorms::diagnostics),
                pivot_magnitude: pivot,
                matrix_norm,
            }));
        }
    }
    Err(LandSurfaceEnergyError::NumericalAcceptedResidual)
}

#[derive(Clone, Debug, PartialEq)]
pub struct CoveredPotentialFinalTransaction {
    pub potential: Box<CoveredColumnCandidate>,
    pub final_pass: Box<CoveredColumnCandidate>,
}

/// Execute the owner-uncapped pass and the fixed-cap pass from the same
/// immutable beginning problem. The supplied cap batch must preserve every
/// potential request identity and amount exactly.
pub fn execute_covered_potential_final(
    beginning: &CoveredColumnInputs,
    potential_initial_trial: Vec<f64>,
    caps: &CoveredWaterCaps,
    final_initial_trial: Vec<f64>,
) -> Result<CoveredPotentialFinalTransaction, LandSurfaceEnergyError> {
    let potential = match solve_covered_column(beginning, None, potential_initial_trial)? {
        CoveredColumnSolveOutcome::Accepted(value) => value,
        CoveredColumnSolveOutcome::Rejected(_) => {
            return Err(LandSurfaceEnergyError::NumericalAcceptedResidual);
        }
    };
    for source in &potential.root_water {
        let cap = caps
            .root
            .get(&(source.occupancy_id.clone(), source.layer_id.clone()))
            .ok_or(LandSurfaceEnergyError::water_cardinality(
                "missing_potential_root_request_identity",
            ))?;
        let potential_rate =
            source.request_kg_m2_stand_ground / (beginning.tile_fraction * beginning.interval_s);
        if cap.request_rate_kg_m2_tile_s != potential_rate {
            return Err(LandSurfaceEnergyError::water_identity(
                "changed_potential_root_request",
            ));
        }
    }
    let ground_potential_rate = potential.ground_water.request_kg_m2_stand_ground
        / (beginning.tile_fraction * beginning.interval_s);
    if caps.ground.request_rate_kg_m2_tile_s != ground_potential_rate {
        return Err(LandSurfaceEnergyError::water_identity(
            "changed_potential_ground_request",
        ));
    }
    // `beginning`, rather than `potential`, is deliberately passed here.
    let final_pass = match solve_covered_column(beginning, Some(caps), final_initial_trial)? {
        CoveredColumnSolveOutcome::Accepted(value) => value,
        CoveredColumnSolveOutcome::Rejected(_) => {
            return Err(LandSurfaceEnergyError::NumericalAcceptedResidual);
        }
    };
    for source in &final_pass.root_water {
        let authorization = source.authorization_kg_m2_stand_ground.ok_or(
            LandSurfaceEnergyError::water_cardinality("missing_final_root_authorization"),
        )?;
        if source.finalized_use_kg_m2_stand_ground > authorization
            || authorization > source.request_kg_m2_stand_ground
        {
            return Err(LandSurfaceEnergyError::water_bound("root_D/A/F"));
        }
    }
    let ground_authorization = final_pass
        .ground_water
        .authorization_kg_m2_stand_ground
        .ok_or(LandSurfaceEnergyError::water_cardinality(
            "missing_final_ground_authorization",
        ))?;
    if final_pass.ground_water.finalized_use_kg_m2_stand_ground > ground_authorization
        || ground_authorization > final_pass.ground_water.request_kg_m2_stand_ground
    {
        return Err(LandSurfaceEnergyError::water_bound("ground_D/A/F"));
    }
    Ok(CoveredPotentialFinalTransaction {
        potential,
        final_pass,
    })
}

#[cfg(test)]
#[path = "solver_tests.rs"]
mod tests;
