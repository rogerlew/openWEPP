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
use crate::{
    LandSurfaceEnergyError, NormalizedResidual, OfeId, ResidualUnit, Sha256Digest, StepNorms,
};
use openwepp_kernel_contract::TileId;
use serde::Serialize;
use sha2::{Digest, Sha256};
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
    let authorized_vapor = match branch {
        WaterBranch::AuthorizationActiveOrTie => authorization_cap_rate_kg_m2_tile_s.ok_or(
            LandSurfaceEnergyError::water_cardinality("frozen_cap_without_authorization"),
        )?,
        WaterBranch::ConstitutiveLaw | WaterBranch::Condensation => law,
    };
    let uses_surface_store = !(problem.class == SurfaceClassKind::BareMineralSoil
        && problem.surface_liquid_kg_m2_tile == 0.0);
    // A positive surface-store vapor export is physically bounded by the
    // water present at the beginning of this carrier support.  This local
    // material bound applies even during the uncapped request evaluation;
    // `request_rate` retains the raw constitutive opportunity for the
    // hydrology authorization transaction.
    let final_vapor = if uses_surface_store && authorized_vapor > 0.0 {
        authorized_vapor.min(problem.surface_liquid_kg_m2_tile / problem.interval_s)
    } else {
        authorized_vapor
    };
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
    let finalized_use = final_vapor.max(0.0) * problem.tile_fraction * problem.interval_s;
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

include!("solver_covered_evaluation.rs");
include!("solver_covered_solve.rs");

#[cfg(test)]
#[path = "solver_tests.rs"]
mod tests;
