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

use crate::LandSurfaceEnergyError;
use crate::physics::{
    AIR_HEAT_CAPACITY_J_KG_K, BandDirectionalFluxes, BareSoilVaporOperands, BareSoilVaporResult,
    DRY_AIR_GAS_CONSTANT_J_KG_K, NeutralResistances, OpenNeutralGeometry, REFERENCE_TEMPERATURE_K,
    STEFAN_BOLTZMANN_W_M2_K4, WATER_HEAT_CAPACITY_J_KG_K, bare_soil_vapor, energy_tolerance,
    harmonic_interface_conductance_w_m2_k, litter_relative_humidity, open_neutral_resistances,
    partition_ground_shortwave, saturation_specific_humidity, vapor_export_w_m2,
};
use std::collections::{BTreeMap, BTreeSet};

const MAX_NEWTON_ITERATIONS: u32 = 50;
const MAX_BACKTRACKING_HALVINGS: u32 = 20;
const TEMPERATURE_STEP_TOLERANCE_K: f64 = 1.0e-8;
const PIVOT_MULTIPLIER: f64 = 64.0;

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

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
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
            return Err(LandSurfaceEnergyError::Topology("soil_nodes"));
        }
        for node in &self.soil_nodes {
            if node.layer_id.is_empty() {
                return Err(LandSurfaceEnergyError::Topology("soil_layer_id"));
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
        return Err(LandSurfaceEnergyError::Topology("open_trial_shape"));
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
            return Err(LandSurfaceEnergyError::WaterIdentityOrBound(
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
            LandSurfaceEnergyError::WaterIdentityOrBound("frozen_cap_without_authorization"),
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
        return Err(LandSurfaceEnergyError::WaterIdentityOrBound(
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
        authorization_amount.ok_or(LandSurfaceEnergyError::WaterIdentityOrBound(
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

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum NumericalFailureKind {
    SingularPivot,
    BacktrackingLimit,
    IterationLimit,
}

#[derive(Clone, Debug, PartialEq)]
pub struct NumericalFailure {
    pub kind: NumericalFailureKind,
    pub iterations: u32,
    pub normalized_residuals: Vec<f64>,
    pub backtracking_count: u32,
    pub step_norm: Option<f64>,
    pub pivot_magnitude: Option<f64>,
    pub matrix_norm: Option<f64>,
}

#[derive(Clone, Debug, PartialEq)]
pub enum NormalizedSolveOutcome<D> {
    Accepted {
        solution: Vec<f64>,
        detail: D,
        iterations: u32,
        residual_norm_history: Vec<f64>,
        backtracking_count: u32,
        step_norm: f64,
        pivot_magnitude: Option<f64>,
        matrix_norm: Option<f64>,
    },
    Rejected(NumericalFailure),
}

#[derive(Debug)]
struct SingularEvidence {
    pivot: f64,
    matrix_norm: f64,
}

fn solve_linear(
    matrix: &[Vec<f64>],
    rhs: &[f64],
) -> Result<(Vec<f64>, f64, f64), SingularEvidence> {
    let n = rhs.len();
    let matrix_norm = matrix
        .iter()
        .map(|row| row.iter().map(|value| value.abs()).sum::<f64>())
        .fold(0.0, f64::max);
    let threshold = PIVOT_MULTIPLIER * f64::EPSILON * matrix_norm;
    let mut a = matrix.to_vec();
    let mut b = rhs.to_vec();
    let mut minimum_pivot = f64::INFINITY;
    for column in 0..n {
        let mut pivot_row = column;
        for row in column + 1..n {
            if a[row][column].abs() > a[pivot_row][column].abs() {
                pivot_row = row;
            }
        }
        let pivot = a[pivot_row][column].abs();
        minimum_pivot = minimum_pivot.min(pivot);
        if !pivot.is_finite() || pivot < threshold {
            return Err(SingularEvidence { pivot, matrix_norm });
        }
        if pivot_row != column {
            a.swap(column, pivot_row);
            b.swap(column, pivot_row);
        }
        for row in column + 1..n {
            let factor = a[row][column] / a[column][column];
            a[row][column] = 0.0;
            for inner in column + 1..n {
                a[row][inner] -= factor * a[column][inner];
            }
            b[row] -= factor * b[column];
        }
    }
    let mut solution = vec![0.0; n];
    for row in (0..n).rev() {
        let tail: f64 = a[row][row + 1..]
            .iter()
            .zip(solution[row + 1..].iter())
            .map(|(coefficient, value)| coefficient * value)
            .sum();
        solution[row] = (b[row] - tail) / a[row][row];
    }
    Ok((solution, minimum_pivot, matrix_norm))
}

/// Frozen centered-difference Newton algorithm shared by open and joint columns.
pub fn solve_normalized_system<D, B, E, V, F>(
    mut evaluator: E,
    initial: Vec<f64>,
    unit_scales: &[f64],
    mut valid_trial: V,
    mut freeze_branches: F,
) -> Result<NormalizedSolveOutcome<D>, LandSurfaceEnergyError>
where
    D: Clone,
    E: FnMut(&[f64], Option<&B>) -> Result<(Vec<f64>, D), LandSurfaceEnergyError>,
    V: FnMut(&[f64]) -> bool,
    F: FnMut(&D) -> B,
    B: Clone,
{
    if initial.len() != unit_scales.len() || initial.is_empty() {
        return Err(LandSurfaceEnergyError::Topology("normalized_solver_shape"));
    }
    let mut x = initial;
    let mut last_step = None;
    let mut backtracking_count = 0;
    let mut pivot = None;
    let mut matrix_norm = None;
    let mut history = Vec::new();
    for iteration in 0..=MAX_NEWTON_ITERATIONS {
        let (normalized, detail) = evaluator(&x, None)?;
        if normalized.len() != x.len() || normalized.iter().any(|value| !value.is_finite()) {
            return Err(LandSurfaceEnergyError::ConstitutiveDomain(
                "normalized_residual_shape_or_domain",
            ));
        }
        let norm = normalized
            .iter()
            .map(|value| value.abs())
            .fold(0.0, f64::max);
        history.push(norm);
        if norm <= 1.0 && last_step.is_some_and(|step| step <= TEMPERATURE_STEP_TOLERANCE_K) {
            return Ok(NormalizedSolveOutcome::Accepted {
                solution: x,
                detail,
                iterations: iteration,
                residual_norm_history: history,
                backtracking_count,
                step_norm: last_step.unwrap_or(0.0),
                pivot_magnitude: pivot,
                matrix_norm,
            });
        }
        if iteration == MAX_NEWTON_ITERATIONS {
            return Ok(NormalizedSolveOutcome::Rejected(NumericalFailure {
                kind: NumericalFailureKind::IterationLimit,
                iterations: iteration,
                normalized_residuals: normalized,
                backtracking_count,
                step_norm: last_step,
                pivot_magnitude: pivot,
                matrix_norm,
            }));
        }
        let frozen = freeze_branches(&detail);
        let perturbations: Vec<f64> = x
            .iter()
            .zip(unit_scales.iter())
            .map(|(value, scale)| f64::EPSILON.sqrt() * value.abs().max(*scale))
            .collect();
        let mut jacobian = vec![vec![0.0; x.len()]; x.len()];
        for column in 0..x.len() {
            let mut minus = x.clone();
            let mut plus = x.clone();
            minus[column] -= perturbations[column];
            plus[column] += perturbations[column];
            let (minus_residual, _) = evaluator(&minus, Some(&frozen))?;
            let (plus_residual, _) = evaluator(&plus, Some(&frozen))?;
            for row in 0..x.len() {
                jacobian[row][column] =
                    (plus_residual[row] - minus_residual[row]) / (2.0 * perturbations[column]);
            }
        }
        let right_hand_side: Vec<f64> = normalized.iter().map(|value| -value).collect();
        let (delta, current_pivot, current_matrix_norm) =
            match solve_linear(&jacobian, &right_hand_side) {
                Ok(value) => value,
                Err(evidence) => {
                    return Ok(NormalizedSolveOutcome::Rejected(NumericalFailure {
                        kind: NumericalFailureKind::SingularPivot,
                        iterations: iteration,
                        normalized_residuals: normalized,
                        backtracking_count,
                        step_norm: last_step,
                        pivot_magnitude: Some(evidence.pivot),
                        matrix_norm: Some(evidence.matrix_norm),
                    }));
                }
            };
        pivot = Some(current_pivot);
        matrix_norm = Some(current_matrix_norm);
        let prospective_step = delta.iter().map(|value| value.abs()).fold(0.0, f64::max);
        if norm <= 1.0 && prospective_step <= TEMPERATURE_STEP_TOLERANCE_K {
            last_step = Some(prospective_step);
            continue;
        }
        let mut accepted = None;
        for exponent in 0..=MAX_BACKTRACKING_HALVINGS {
            let factor = 0.5_f64.powf(f64::from(exponent));
            let trial: Vec<f64> = x
                .iter()
                .zip(delta.iter())
                .map(|(value, change)| value + factor * change)
                .collect();
            if !valid_trial(&trial) {
                continue;
            }
            let trial_result = evaluator(&trial, None);
            let Ok((trial_residual, _)) = trial_result else {
                continue;
            };
            let trial_norm = trial_residual
                .iter()
                .map(|value| value.abs())
                .fold(0.0, f64::max);
            if trial_norm < norm {
                accepted = Some((trial, factor * prospective_step, exponent));
                break;
            }
        }
        if let Some((trial, step, exponent)) = accepted {
            x = trial;
            last_step = Some(step);
            backtracking_count += exponent;
        } else {
            return Ok(NormalizedSolveOutcome::Rejected(NumericalFailure {
                kind: NumericalFailureKind::BacktrackingLimit,
                iterations: iteration,
                normalized_residuals: normalized,
                backtracking_count: backtracking_count + MAX_BACKTRACKING_HALVINGS,
                step_norm: last_step,
                pivot_magnitude: pivot,
                matrix_norm,
            }));
        }
    }
    Err(LandSurfaceEnergyError::ConstitutiveDomain(
        "unreachable_normalized_solver_state",
    ))
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
        NormalizedSolveOutcome::Rejected(failure) => OpenSurfaceSolveOutcome::Rejected(failure),
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
    pub sun: LeafBiochemicalInputs,
    pub shade: LeafBiochemicalInputs,
    pub biochemical: BiochemicalConstants,
    pub stem_area_m2_m2_tile: f64,
    pub stem_absorbed_shortwave_w_m2_tile: f64,
    pub wet_fraction: f64,
    pub canopy_liquid_kg_m2_tile: f64,
    pub gb_leaf_m_s: f64,
    pub gb_wet_m_s: f64,
    pub gb_stem_m_s: f64,
    pub emax_sun_kg_m2_s: f64,
    pub emax_shade_kg_m2_s: f64,
    pub lai: f64,
    pub sai: f64,
    pub clumping_index: f64,
    pub k1_max: f64,
    pub k2_max: f64,
    pub k3_max_m_s: f64,
    pub stem_to_leaf_path_m: f64,
    pub height_m: f64,
    pub root_to_leaf_area: f64,
    pub p50_leaf_mm: f64,
    pub p50_xylem_mm: f64,
    pub p50_root_mm: f64,
    pub vulnerability_exponent: f64,
    pub root_layers: Vec<RootHydraulicLayer>,
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
    pub medlyn_g1_kpa_sqrt: f64,
    pub g0_umol_m2_s: f64,
    pub canopy_to_atmosphere_heat_resistance_s_m: f64,
    pub canopy_to_atmosphere_vapor_resistance_s_m: f64,
    pub latent_heat_j_kg: f64,
    pub under_canopy_geometry: crate::physics::UnderCanopyGeometry,
    pub ground: OpenSurfaceProblem,
    pub occupancies: Vec<CoveredOccupancyInputs>,
}

#[derive(Clone, Copy, Debug, PartialEq)]
struct LeafTrialState {
    surface_q: f64,
    rs_s_m: f64,
    ci_pa: f64,
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

fn leaf_trial_state(
    inputs: LeafBiochemicalInputs,
    p: BiochemicalConstants,
    temperature: f64,
    qcan: f64,
    beta: f64,
    column: &CoveredColumnInputs,
    gb_leaf: f64,
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
    let residual = |ci: f64| -> Result<(f64, f64), LandSurfaceEnergyError> {
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
        let rb = 1.0 / gb_leaf;
        let cs = column.ca_pa - 1.4 * rb * MOLAR_GAS_CONSTANT * temperature * an * 1.0e-6;
        if cs <= 0.0 {
            return Err(LandSurfaceEnergyError::ConstitutiveDomain("surface_co2"));
        }
        let potential = if an <= 0.0 {
            column.g0_umol_m2_s
        } else {
            column.g0_umol_m2_s
                + 1.6 * (1.0 + column.medlyn_g1_kpa_sqrt / vpd.sqrt()) * an
                    / (cs / column.pressure_pa)
        };
        let gs = column.g0_umol_m2_s + beta * (potential - column.g0_umol_m2_s);
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
        return Ok(LeafTrialState {
            surface_q: qsurface,
            rs_s_m: rs,
            ci_pa: a,
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
    Ok(LeafTrialState {
        surface_q: qsurface,
        rs_s_m: rs,
        ci_pa: b,
    })
}

#[must_use]
fn vulnerability(potential: f64, p50: f64, exponent: f64) -> f64 {
    2.0_f64.powf(-(potential / p50).powf(exponent))
}

/// Evaluate the concrete per-occupancy V8 hydraulic and component-energy
/// block at the shared current-trial canopy-air and longwave boundaries.
#[derive(Clone, Debug, PartialEq)]
pub struct CoveredOccupancyEvaluation {
    pub residuals: Vec<f64>,
    pub tolerances: Vec<f64>,
    pub source_water: Vec<SourceWaterFlux>,
    pub canopy_sensible_w_m2: f64,
    pub canopy_vapor_kg_m2_s: f64,
    pub wet_vapor_kg_m2_s: f64,
    pub wet_branch: WaterBranch,
    pub component_temperatures_k: [f64; 4],
    pub ci_pa: [f64; 2],
}

struct CoveredOccupancyTrialContext<'a> {
    column: &'a CoveredColumnInputs,
    canopy_air_temperature_k: f64,
    canopy_air_q: f64,
    component_longwave_w_m2: [f64; 4],
    caps: Option<&'a CoveredWaterCaps>,
    frozen: Option<&'a CoveredFrozenBranches>,
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
        return Err(LandSurfaceEnergyError::Topology("covered_occupancy_trial"));
    }
    let (psi_sun, psi_shade, psi_stem, psi_root) = (block[0], block[1], block[2], block[3]);
    let (beta_sun, beta_shade) = (block[4], block[5]);
    let (tsun, tshade, twet, tstem) = (block[6], block[7], block[8], block[9]);
    let rho = column.pressure_pa / (DRY_AIR_GAS_CONSTANT_J_KG_K * canopy_air_temperature_k);
    let dry_sun = occupancy.sun.leaf_area_m2_m2_tile * (1.0 - occupancy.wet_fraction);
    let dry_shade = occupancy.shade.leaf_area_m2_m2_tile * (1.0 - occupancy.wet_fraction);
    let wet_area = occupancy.wet_fraction
        * (occupancy.sun.leaf_area_m2_m2_tile
            + occupancy.shade.leaf_area_m2_m2_tile
            + occupancy.stem_area_m2_m2_tile);
    let dry_stem = (1.0 - occupancy.wet_fraction) * occupancy.stem_area_m2_m2_tile;
    let sun = leaf_trial_state(
        occupancy.sun,
        occupancy.biochemical,
        tsun,
        canopy_air_q,
        beta_sun,
        column,
        occupancy.gb_leaf_m_s,
    )?;
    let shade = leaf_trial_state(
        occupancy.shade,
        occupancy.biochemical,
        tshade,
        canopy_air_q,
        beta_shade,
        column,
        occupancy.gb_leaf_m_s,
    )?;
    let sun_e =
        rho * (sun.surface_q - canopy_air_q) / (1.0 / occupancy.gb_leaf_m_s + sun.rs_s_m) * dry_sun;
    let shade_e = rho * (shade.surface_q - canopy_air_q)
        / (1.0 / occupancy.gb_leaf_m_s + shade.rs_s_m)
        * dry_shade;
    let wet_potential = rho
        * occupancy.gb_wet_m_s
        * (canopy_saturation_q(twet, column.pressure_pa)? - canopy_air_q)
        * wet_area;
    let wet_cap = occupancy.canopy_liquid_kg_m2_tile / column.interval_s;
    let natural_wet_branch = if wet_potential >= 0.0 && wet_cap <= wet_potential {
        WaterBranch::AuthorizationActiveOrTie
    } else if wet_potential < 0.0 {
        WaterBranch::Condensation
    } else {
        WaterBranch::ConstitutiveLaw
    };
    let wet_branch = context
        .frozen
        .and_then(|value| value.wet.get(&occupancy.occupancy_id).copied())
        .unwrap_or(natural_wet_branch);
    let wet_e = if wet_branch == WaterBranch::AuthorizationActiveOrTie {
        wet_cap
    } else {
        wet_potential
    };
    let leaf_factor = occupancy.k1_max / occupancy.stem_to_leaf_path_m;
    let q1sun = leaf_factor
        * occupancy.sun.leaf_area_m2_m2_tile
        * vulnerability(
            psi_stem,
            occupancy.p50_xylem_mm,
            occupancy.vulnerability_exponent,
        )
        * (psi_stem - psi_sun);
    let q1shade = leaf_factor
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
            cap_rate.ok_or(LandSurfaceEnergyError::WaterIdentityOrBound(
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
            authorization.ok_or(LandSurfaceEnergyError::WaterIdentityOrBound(
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
    let residuals = vec![
        sun_e - q1sun,
        shade_e - q1shade,
        sun_e
            - occupancy.emax_sun_kg_m2_s
                * vulnerability(
                    psi_sun,
                    occupancy.p50_leaf_mm,
                    occupancy.vulnerability_exponent,
                ),
        shade_e
            - occupancy.emax_shade_kg_m2_s
                * vulnerability(
                    psi_shade,
                    occupancy.p50_leaf_mm,
                    occupancy.vulnerability_exponent,
                ),
        q1sun + q1shade - q2,
        q2 - root_source_sum,
        occupancy.sun.absorbed_shortwave_w_m2_tile * (1.0 - occupancy.wet_fraction)
            + component_longwave_w_m2[0]
            - sun_h
            - column.latent_heat_j_kg * sun_e,
        occupancy.shade.absorbed_shortwave_w_m2_tile * (1.0 - occupancy.wet_fraction)
            + component_longwave_w_m2[1]
            - shade_h
            - column.latent_heat_j_kg * shade_e,
        occupancy.wet_fraction
            * (occupancy.sun.absorbed_shortwave_w_m2_tile
                + occupancy.shade.absorbed_shortwave_w_m2_tile
                + occupancy.stem_absorbed_shortwave_w_m2_tile)
            + component_longwave_w_m2[2]
            - wet_h
            - column.latent_heat_j_kg * wet_e,
        (1.0 - occupancy.wet_fraction) * occupancy.stem_absorbed_shortwave_w_m2_tile
            + component_longwave_w_m2[3]
            - stem_h,
    ];
    let water_scale = occupancy
        .emax_sun_kg_m2_s
        .max(occupancy.emax_shade_kg_m2_s)
        .max(q1sun.abs())
        .max(q1shade.abs())
        .max(q2.abs())
        .max(root_source_sum.abs());
    let component_operands = [
        occupancy.sun.absorbed_shortwave_w_m2_tile * (1.0 - occupancy.wet_fraction),
        occupancy.shade.absorbed_shortwave_w_m2_tile * (1.0 - occupancy.wet_fraction),
        occupancy.wet_fraction
            * (occupancy.sun.absorbed_shortwave_w_m2_tile
                + occupancy.shade.absorbed_shortwave_w_m2_tile
                + occupancy.stem_absorbed_shortwave_w_m2_tile),
        (1.0 - occupancy.wet_fraction) * occupancy.stem_absorbed_shortwave_w_m2_tile,
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
        crate::physics::energy_tolerance(
            component_operands[index].abs()
                + component_longwave_w_m2[index].abs()
                + sensible[index].abs()
                + latent[index].abs(),
        )
    }));
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
    let context = CoveredOccupancyTrialContext {
        column,
        canopy_air_temperature_k,
        canopy_air_q,
        component_longwave_w_m2,
        caps: None,
        frozen: None,
    };
    Ok(evaluate_covered_occupancy(&context, occupancy, block)?.residuals)
}

#[derive(Clone, Debug, PartialEq)]
pub struct CoveredColumnEvaluation {
    pub raw_residuals: Vec<f64>,
    pub normalized_residuals: Vec<f64>,
    pub tolerances: Vec<f64>,
    pub occupancies: Vec<CoveredOccupancyEvaluation>,
    pub canopy_air_temperature_k: f64,
    pub canopy_air_specific_humidity_kg_kg: f64,
    pub ground_temperature_k: f64,
    pub soil_temperature_k: Vec<f64>,
    pub ground_water: GroundWaterFlux,
    pub ground_heat_cn_w_m2_tile: Vec<f64>,
    pub ground_storage_w_m2_tile: f64,
    pub ending_surface_enthalpy_j_m2_tile: f64,
    pub whole_column_longwave: crate::physics::CanopyLongwaveResult,
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
        return Err(LandSurfaceEnergyError::WaterIdentityOrBound(
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
                || value.authorization_rate_kg_m2_tile_s < 0.0
                || value.request_rate_kg_m2_tile_s < value.authorization_rate_kg_m2_tile_s
        })
    {
        return Err(LandSurfaceEnergyError::WaterIdentityOrBound(
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

pub fn evaluate_covered_column(
    column: &CoveredColumnInputs,
    trial: &[f64],
    caps: Option<&CoveredWaterCaps>,
    frozen: Option<&CoveredFrozenBranches>,
) -> Result<CoveredColumnEvaluation, LandSurfaceEnergyError> {
    if column.occupancies.is_empty() || column.ground.soil_nodes.is_empty() {
        return Err(LandSurfaceEnergyError::Topology("covered_column"));
    }
    validate_covered_caps(column, caps)?;
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
    let longwave_layers: Vec<_> = column
        .occupancies
        .iter()
        .enumerate()
        .map(|(index, occupancy)| {
            let block = &trial[index * 10..(index + 1) * 10];
            let dry_sun = occupancy.sun.leaf_area_m2_m2_tile * (1.0 - occupancy.wet_fraction);
            let dry_shade = occupancy.shade.leaf_area_m2_m2_tile * (1.0 - occupancy.wet_fraction);
            let wet = occupancy.wet_fraction
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
                    (1.0 - occupancy.wet_fraction) * occupancy.stem_area_m2_m2_tile,
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
        };
        let result =
            evaluate_covered_occupancy(&context, occupancy, &trial[index * 10..(index + 1) * 10])?;
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
        caps.ok_or(LandSurfaceEnergyError::WaterIdentityOrBound(
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
        return Err(LandSurfaceEnergyError::WaterIdentityOrBound(
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
        ground_authorization.ok_or(LandSurfaceEnergyError::WaterIdentityOrBound(
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
            && self.ci_pa <= 1.0e-8
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
        let norm = detail
            .normalized_residuals
            .iter()
            .map(|value| value.abs())
            .fold(0.0, f64::max);
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
            return Ok(CoveredColumnSolveOutcome::Rejected(NumericalFailure {
                kind: NumericalFailureKind::IterationLimit,
                iterations: iteration,
                normalized_residuals: detail.normalized_residuals,
                backtracking_count,
                step_norm: last_steps.map(|value: CoveredStepNorms| {
                    value
                        .temperature_k
                        .max(value.hydraulic_mm)
                        .max(value.beta)
                        .max(value.humidity_kg_kg)
                        .max(value.ci_pa)
                }),
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
                return Ok(CoveredColumnSolveOutcome::Rejected(NumericalFailure {
                    kind: NumericalFailureKind::SingularPivot,
                    iterations: iteration,
                    normalized_residuals: detail.normalized_residuals,
                    backtracking_count,
                    step_norm: None,
                    pivot_magnitude: Some(evidence.pivot),
                    matrix_norm: Some(evidence.matrix_norm),
                }));
            }
        };
        pivot = Some(current_pivot);
        matrix_norm = Some(current_matrix_norm);
        // The admitted algorithm takes a valid full Newton step when the
        // current residual already passes and that prospective step satisfies
        // every dimensional step criterion. Requiring another residual-norm
        // decrease here would incorrectly turn a converged trajectory into a
        // backtracking failure at floating-point resolution.
        if norm <= 1.0 {
            let prospective: Vec<f64> = x
                .iter()
                .zip(delta.iter())
                .map(|(value, change)| value + change)
                .collect();
            if covered_trial_is_valid(&prospective, beginning.occupancies.len()) {
                if let Ok(prospective_detail) =
                    evaluate_covered_column(beginning, &prospective, caps, Some(&frozen))
                {
                    let prospective_steps = covered_step_norms(
                        &delta,
                        beginning.occupancies.len(),
                        &detail,
                        &prospective_detail,
                    );
                    if prospective_steps.accepted() {
                        x = prospective;
                        last_steps = Some(prospective_steps);
                        continue;
                    }
                }
            }
        }
        let mut accepted = None;
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
            let Ok(trial_detail) = evaluate_covered_column(beginning, &trial, caps, Some(&frozen))
            else {
                continue;
            };
            let trial_norm = trial_detail
                .normalized_residuals
                .iter()
                .map(|value| value.abs())
                .fold(0.0, f64::max);
            if trial_norm < norm {
                let applied: Vec<f64> = delta.iter().map(|value| factor * value).collect();
                let steps = covered_step_norms(
                    &applied,
                    beginning.occupancies.len(),
                    &detail,
                    &trial_detail,
                );
                accepted = Some((trial, steps, exponent));
                break;
            }
        }
        if let Some((trial, steps, exponent)) = accepted {
            x = trial;
            last_steps = Some(steps);
            backtracking_count += exponent;
        } else {
            return Ok(CoveredColumnSolveOutcome::Rejected(NumericalFailure {
                kind: NumericalFailureKind::BacktrackingLimit,
                iterations: iteration,
                normalized_residuals: detail.normalized_residuals,
                backtracking_count: backtracking_count + MAX_BACKTRACKING_HALVINGS,
                step_norm: None,
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
            .ok_or(LandSurfaceEnergyError::WaterIdentityOrBound(
                "missing_potential_root_request_identity",
            ))?;
        let potential_rate =
            source.request_kg_m2_stand_ground / (beginning.tile_fraction * beginning.interval_s);
        if cap.request_rate_kg_m2_tile_s != potential_rate {
            return Err(LandSurfaceEnergyError::WaterIdentityOrBound(
                "changed_potential_root_request",
            ));
        }
    }
    let ground_potential_rate = potential.ground_water.request_kg_m2_stand_ground
        / (beginning.tile_fraction * beginning.interval_s);
    if caps.ground.request_rate_kg_m2_tile_s != ground_potential_rate {
        return Err(LandSurfaceEnergyError::WaterIdentityOrBound(
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
            LandSurfaceEnergyError::WaterIdentityOrBound("missing_final_root_authorization"),
        )?;
        if source.finalized_use_kg_m2_stand_ground > authorization
            || authorization > source.request_kg_m2_stand_ground
        {
            return Err(LandSurfaceEnergyError::WaterIdentityOrBound("root_D/A/F"));
        }
    }
    let ground_authorization = final_pass
        .ground_water
        .authorization_kg_m2_stand_ground
        .ok_or(LandSurfaceEnergyError::WaterIdentityOrBound(
            "missing_final_ground_authorization",
        ))?;
    if final_pass.ground_water.finalized_use_kg_m2_stand_ground > ground_authorization
        || ground_authorization > final_pass.ground_water.request_kg_m2_stand_ground
    {
        return Err(LandSurfaceEnergyError::WaterIdentityOrBound("ground_D/A/F"));
    }
    Ok(CoveredPotentialFinalTransaction {
        potential,
        final_pass,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn four_layer_problem() -> OpenSurfaceProblem {
        OpenSurfaceProblem {
            interval_s: 1_800.0,
            tile_fraction: 1.0,
            class: SurfaceClassKind::BareMineralSoil,
            storage_branch: SurfaceStorageBranch::FiniteCapacity,
            terminal_shortwave_w_m2_tile: BandDirectionalFluxes {
                direct_vis: 91.0,
                diffuse_vis: 31.0,
                direct_nir: 117.0,
                diffuse_nir: 39.0,
            },
            surface_vis_albedo: 0.18,
            surface_nir_albedo: 0.31,
            surface_emissivity: 1.0,
            surface_depth_m: 0.02,
            surface_conductivity_w_m_k: 0.75,
            surface_dry_heat_capacity_j_m2_k: 42_000.0,
            litter_capacity_kg_m2_tile: None,
            open_geometry: OpenNeutralGeometry {
                reference_height_m: 20.0,
                roughness_momentum_m: 0.12,
                roughness_heat_m: 0.015,
                roughness_vapor_m: 0.010,
            },
            air_temperature_k: 294.0,
            air_specific_humidity_kg_kg: 0.0095,
            air_pressure_pa: 93_000.0,
            reference_wind_m_s: 2.4,
            atmospheric_downward_longwave_w_m2: 335.0,
            surface_liquid_kg_m2_tile: 0.0,
            surface_enthalpy_j_m2_tile: 42_000.0 * (295.0 - REFERENCE_TEMPERATURE_K),
            surface_temperature_warm_start_k: 295.0,
            bare_soil: Some(BareSoilParameters {
                top_layer_liquid_kg_m2: 26.0,
                top_layer_ice_kg_m2: 0.0,
                porosity: 0.46,
                saturated_matric_potential_mm: -120.0,
                clapp_hornberger_b: 4.05,
                theta_initial: 0.22,
            }),
            soil_nodes: (0..4)
                .map(|index| SoilThermalNodeOperands {
                    layer_id: format!("thermal-{}", index + 1),
                    depth_m: 0.08 + 0.05 * f64::from(index),
                    conductivity_w_m_k: 1.1 + 0.12 * f64::from(index),
                    heat_capacity_j_m2_k: 120_000.0 + 35_000.0 * f64::from(index),
                    beginning_temperature_k: 291.5 - 1.1 * f64::from(index),
                })
                .collect(),
        }
    }

    #[test]
    fn fixed_cap_rebuild_matches_independent_four_layer_vector() {
        let problem = four_layer_problem();
        let cap = 0.000_053_040_160_893_323_02;
        let result = solve_open_surface(&problem, Some(cap), None).expect("valid solve");
        let OpenSurfaceSolveOutcome::Accepted(accepted) = result else {
            panic!("expected accepted vector");
        };
        let expected = [
            293.055_973_826_482_8,
            291.973_087_253_131_9,
            290.461_060_535_837_25,
            289.318_132_686_869_4,
            288.259_177_307_246_15,
        ];
        for (actual, expected) in accepted.solution.iter().zip(expected) {
            assert!(
                (actual - expected).abs() < 2.0e-10,
                "{actual} != {expected}"
            );
        }
        assert_eq!(
            accepted.evaluation.water.branch,
            WaterBranch::AuthorizationActiveOrTie
        );
        assert_eq!(
            accepted.evaluation.water.finalized_use_kg_m2_stand_ground,
            cap * problem.interval_s
        );
    }

    #[test]
    fn alternate_warm_start_converges_to_same_accepted_state() {
        let problem = four_layer_problem();
        let cap = 0.000_053_040_160_893_323_02;
        let first = solve_open_surface(&problem, Some(cap), None).expect("first");
        let alternate = vec![298.0, 289.0, 288.0, 287.0, 286.0];
        let second = solve_open_surface(&problem, Some(cap), Some(alternate)).expect("second");
        let (OpenSurfaceSolveOutcome::Accepted(first), OpenSurfaceSolveOutcome::Accepted(second)) =
            (first, second)
        else {
            panic!("both starts must converge");
        };
        for (left, right) in first.solution.iter().zip(second.solution.iter()) {
            assert!((left - right).abs() < 2.0e-10);
        }
    }

    #[test]
    fn potential_and_final_do_not_mutate_beginning_problem() {
        let problem = four_layer_problem();
        let beginning = problem.clone();
        let potential = solve_open_surface(&problem, None, None).expect("potential");
        assert!(matches!(potential, OpenSurfaceSolveOutcome::Accepted(_)));
        let final_pass =
            solve_open_surface(&problem, Some(0.000_053_040_160_893_323_02), None).expect("capped");
        assert!(matches!(final_pass, OpenSurfaceSolveOutcome::Accepted(_)));
        assert_eq!(problem, beginning);
    }

    #[test]
    fn equality_is_cap_active() {
        let problem = four_layer_problem();
        let trial = problem.initial_trial();
        let potential = evaluate_open_surface(&problem, &trial, None, None).expect("potential");
        let capped = evaluate_open_surface(
            &problem,
            &trial,
            Some(potential.water.law_kg_m2_tile_s),
            None,
        )
        .expect("tie");
        assert_eq!(capped.water.branch, WaterBranch::AuthorizationActiveOrTie);
    }

    #[test]
    fn covered_v8_block_matches_frozen_joint_solution() {
        let ground = OpenSurfaceProblem {
            interval_s: 1_800.0,
            tile_fraction: 0.38,
            class: SurfaceClassKind::ForestLitter,
            storage_branch: SurfaceStorageBranch::FiniteCapacity,
            terminal_shortwave_w_m2_tile: BandDirectionalFluxes {
                direct_vis: 47.412_973_012_352_3,
                diffuse_vis: 8.705_736_606_981_51,
                direct_nir: 41.052_696_144_841_63,
                diffuse_nir: 52.084_944_358_632_505,
            },
            surface_vis_albedo: 0.12,
            surface_nir_albedo: 0.24,
            surface_emissivity: 1.0,
            surface_depth_m: 0.04,
            surface_conductivity_w_m_k: 0.103,
            surface_dry_heat_capacity_j_m2_k: 3_235.68,
            litter_capacity_kg_m2_tile: Some(6.0),
            open_geometry: OpenNeutralGeometry {
                reference_height_m: 24.0,
                roughness_momentum_m: 1.25,
                roughness_heat_m: 0.12,
                roughness_vapor_m: 0.08,
            },
            air_temperature_k: 296.0,
            air_specific_humidity_kg_kg: 0.0102,
            air_pressure_pa: 101_325.0,
            reference_wind_m_s: 3.7,
            atmospheric_downward_longwave_w_m2: 395.0,
            surface_liquid_kg_m2_tile: 4.0,
            surface_enthalpy_j_m2_tile: 439_352.808_000_000_5,
            surface_temperature_warm_start_k: 295.0,
            bare_soil: None,
            soil_nodes: vec![
                SoilThermalNodeOperands {
                    layer_id: "thermal-1".into(),
                    depth_m: 0.08,
                    conductivity_w_m_k: 1.1,
                    heat_capacity_j_m2_k: 120_000.0,
                    beginning_temperature_k: 291.5,
                },
                SoilThermalNodeOperands {
                    layer_id: "thermal-2".into(),
                    depth_m: 0.18,
                    conductivity_w_m_k: 1.35,
                    heat_capacity_j_m2_k: 180_000.0,
                    beginning_temperature_k: 289.8,
                },
            ],
        };
        let mut column = CoveredColumnInputs {
            interval_s: 1_800.0,
            tile_fraction: 0.38,
            pressure_pa: 101_325.0,
            air_temperature_k: 296.0,
            air_specific_humidity_kg_kg: 0.0102,
            reference_wind_m_s: 3.7,
            atmospheric_downward_longwave_w_m2: 395.0,
            ca_pa: 42.0,
            medlyn_g1_kpa_sqrt: 3.5,
            g0_umol_m2_s: 25.0,
            canopy_to_atmosphere_heat_resistance_s_m: 20.992_293_151_292_14,
            canopy_to_atmosphere_vapor_resistance_s_m: 22.734_132_598_127_985,
            latent_heat_j_kg: 2_501_000.0,
            under_canopy_geometry: crate::physics::UnderCanopyGeometry {
                canopy_height_m: 12.5,
                canopy_roughness_m: 1.25,
                reference_height_m: 24.0,
                leaf_area_index: 2.708_333_333_333_333,
            },
            ground,
            occupancies: Vec::new(),
        };
        let biochemical = BiochemicalConstants {
            ha_vcmax_j_mol: 65_330.0,
            hd_vcmax_j_mol: 200_000.0,
            entropy_vcmax_j_mol_k: 650.0,
            ha_jmax_j_mol: 43_540.0,
            hd_jmax_j_mol: 200_000.0,
            entropy_jmax_j_mol_k: 650.0,
            kc25_pa: 40.49,
            ha_kc_j_mol: 79_430.0,
            ko25_pa: 27_840.0,
            ha_ko_j_mol: 36_380.0,
            gamma25_pa: 4.275,
            ha_gamma_j_mol: 37_830.0,
            oxygen_partial_pressure_pa: 20_265.0,
            tp_vcmax_ratio: 0.167,
            electron_quantum_yield: 0.85,
            par_photon_umol_per_j: 4.6,
            electron_curvature: 0.7,
            ac_aj_curvature: 0.98,
            ag_ap_curvature: 0.95,
        };
        let occupancy = CoveredOccupancyInputs {
            occupancy_id: "canopy-rank-0".into(),
            sun: LeafBiochemicalInputs {
                leaf_area_m2_m2_tile: 1.110_267_869_704_946_6,
                absorbed_shortwave_w_m2_tile: 220.671_526_988_526_7,
                absorbed_par_w_m2_leaf: 136.733_826_525_724_48,
                vcmax25: 62.0,
                jmax25: 108.0,
                rd25: 1.15,
            },
            shade: LeafBiochemicalInputs {
                leaf_area_m2_m2_tile: 1.598_065_463_628_386_4,
                absorbed_shortwave_w_m2_tile: 300.708_550_892_603_5,
                absorbed_par_w_m2_leaf: 118.229_071_004_667_92,
                vcmax25: 41.0,
                jmax25: 74.0,
                rd25: 0.81,
            },
            biochemical,
            stem_area_m2_m2_tile: 0.72,
            stem_absorbed_shortwave_w_m2_tile: 185.377_620_426_979_95,
            wet_fraction: 0.37,
            canopy_liquid_kg_m2_tile: 0.018,
            gb_leaf_m_s: 0.035_961_386_715_575_215,
            gb_wet_m_s: 0.019_071_405_305_591_295,
            gb_stem_m_s: 0.013_082_876_106_352_972,
            emax_sun_kg_m2_s: 4.016_697_077_733_990_4e-5,
            emax_shade_kg_m2_s: 1.685_782_614_110_486_5e-5,
            lai: 2.708_333_333_333_333,
            sai: 0.72,
            clumping_index: 0.82,
            k1_max: 1.2e-6,
            k2_max: 4.2e-6,
            k3_max_m_s: 5.0e-5,
            stem_to_leaf_path_m: 1.0,
            height_m: 12.5,
            root_to_leaf_area: 1.8,
            p50_leaf_mm: -9_800.0,
            p50_xylem_mm: -7_200.0,
            p50_root_mm: -14_000.0,
            vulnerability_exponent: 2.0,
            root_layers: vec![
                RootHydraulicLayer {
                    layer_id: "soil-1".into(),
                    accessible: true,
                    frozen: false,
                    root_fraction: 0.62,
                    soil_potential_mm: 100.0,
                    gravity_head_mm: 120.0,
                    z3_m: 0.32,
                    dxroot_m: 0.18,
                    ksoil_m2_s: 6.0e-11,
                },
                RootHydraulicLayer {
                    layer_id: "soil-2".into(),
                    accessible: true,
                    frozen: false,
                    root_fraction: 0.38,
                    soil_potential_mm: 100.0,
                    gravity_head_mm: 360.0,
                    z3_m: 0.55,
                    dxroot_m: 0.24,
                    ksoil_m2_s: 4.5e-11,
                },
                RootHydraulicLayer {
                    layer_id: "soil-dry".into(),
                    accessible: false,
                    frozen: false,
                    root_fraction: 0.0,
                    soil_potential_mm: -9_000.0,
                    gravity_head_mm: 600.0,
                    z3_m: 0.8,
                    dxroot_m: 0.31,
                    ksoil_m2_s: 2.0e-7,
                },
                RootHydraulicLayer {
                    layer_id: "soil-frozen".into(),
                    accessible: true,
                    frozen: true,
                    root_fraction: 0.0,
                    soil_potential_mm: -1_100.0,
                    gravity_head_mm: 740.0,
                    z3_m: 1.1,
                    dxroot_m: 0.4,
                    ksoil_m2_s: 1.0e-7,
                },
            ],
        };
        let block = [
            -16_794.199_307_435_894,
            -16_682.646_177_134_86,
            -16_634.939_805_653_79,
            -4_096.101_073_026_441,
            0.428_451_563_068_840_47,
            0.388_452_456_902_700_7,
            303.532_129_970_361_2,
            303.587_878_184_182_84,
            306.419_495_007_166,
            312.518_870_359_613_3,
        ];
        let residual = evaluate_covered_occupancy_block(
            &column,
            &occupancy,
            &block,
            300.108_675_593_887_5,
            0.013_923_766_548_382_881,
            [
                -24.146_999_444_933_243,
                -34.941_835_646_958_17,
                -56.087_167_507_728_28,
                -29.762_536_844_697_493,
            ],
        )
        .expect("joint block");
        assert!(residual[..6].iter().all(|value| value.abs() < 1.0e-12));
        assert!(residual[6..].iter().all(|value| value.abs() < 1.0e-6));
        column.occupancies = vec![occupancy];
        let full_trial = [
            block.as_slice(),
            &[
                300.108_675_593_887_5,
                0.013_923_766_548_382_881,
                296.005_096_548_963_93,
                291.522_125_408_338_35,
                289.958_387_290_989_04,
            ],
        ]
        .concat();
        let full =
            evaluate_covered_column(&column, &full_trial, None, None).expect("full covered column");
        assert!(
            full.normalized_residuals
                .iter()
                .all(|value| value.abs() <= 1.0)
        );
        let solved =
            solve_covered_column(&column, None, full_trial.clone()).expect("potential solve");
        let CoveredColumnSolveOutcome::Accepted(potential) = solved else {
            panic!("potential must accept");
        };
        let mut root_caps = BTreeMap::new();
        for source in &potential.root_water {
            let request_rate =
                source.request_kg_m2_stand_ground / (column.tile_fraction * column.interval_s);
            let authorization_rate = match source.layer_id.as_str() {
                "soil-1" => 5.449_439_753_166_194e-6,
                "soil-2" => 2.003_239_473_339_757_3e-6,
                _ => 0.0,
            };
            root_caps.insert(
                (source.occupancy_id.clone(), source.layer_id.clone()),
                SourceWaterCap {
                    request_rate_kg_m2_tile_s: request_rate,
                    authorization_rate_kg_m2_tile_s: authorization_rate,
                },
            );
        }
        let caps = CoveredWaterCaps {
            root: root_caps,
            ground: SourceWaterCap {
                request_rate_kg_m2_tile_s: potential.ground_water.request_kg_m2_stand_ground
                    / (column.tile_fraction * column.interval_s),
                authorization_rate_kg_m2_tile_s: 1.226_044_233_320_78e-4,
            },
        };
        let capped_trial = vec![
            -16_824.779_647_297_01,
            -16_712.589_117_241_627,
            -16_664.596_249_631_624,
            -4_125.915_697_953_702,
            0.462_302_155_485_367_2,
            0.424_353_863_538_429,
            305.035_913_166_871_3,
            305.089_706_734_110_45,
            307.913_092_586_148_65,
            314.005_994_994_191_95,
            301.610_676_717_580_6,
            0.012_920_640_609_040_12,
            298.721_285_856_343_5,
            291.602_314_111_137_45,
            289.962_098_689_575_8,
        ];
        let transaction = execute_covered_potential_final(&column, full_trial, &caps, capped_trial)
            .expect("immutable potential/final transaction");
        assert_eq!(
            transaction.final_pass.ground_water.branch,
            WaterBranch::AuthorizationActiveOrTie
        );
        assert!(
            transaction
                .final_pass
                .root_water
                .iter()
                .all(|source| source.finalized_use_kg_m2_stand_ground
                    <= source.authorization_kg_m2_stand_ground.unwrap_or(0.0))
        );

        // The two-rank fixture exercises one shared canopy-air node and the
        // reciprocal longwave network across heterogeneous occupancies.
        let upper = &mut column.occupancies[0];
        upper.sun.absorbed_shortwave_w_m2_tile = 219.583_484_232_463_2;
        upper.sun.absorbed_par_w_m2_leaf = 136.097_574_782_013_34;
        upper.shade.absorbed_shortwave_w_m2_tile = 297.182_430_346_421_4;
        upper.shade.absorbed_par_w_m2_leaf = 116.714_147_486_897_5;
        upper.stem_absorbed_shortwave_w_m2_tile = 183.772_038_359_786_73;
        let mut lower = upper.clone();
        lower.occupancy_id = "canopy-rank-1".into();
        lower.sun.leaf_area_m2_m2_tile = 0.869_597_990_586_524_9;
        lower.sun.absorbed_shortwave_w_m2_tile = 36.606_943_691_269_41;
        lower.sun.absorbed_par_w_m2_leaf = 21.581_281_690_559_077;
        lower.shade.leaf_area_m2_m2_tile = 0.701_235_342_746_808_2;
        lower.shade.absorbed_shortwave_w_m2_tile = 29.370_268_258_774_185;
        lower.shade.absorbed_par_w_m2_leaf = 19.137_976_248_584_64;
        lower.stem_area_m2_m2_tile = 0.417_6;
        lower.stem_absorbed_shortwave_w_m2_tile = 23.961_096_147_421_54;
        lower.lai = 1.570_833_333_333_333;
        lower.sai = 0.417_6;
        lower.clumping_index = 0.91;
        column.occupancies.push(lower);
        column.ground.terminal_shortwave_w_m2_tile = BandDirectionalFluxes {
            direct_vis: 12.572_362_927_904_654,
            diffuse_vis: 2.794_652_935_170_348_4,
            direct_nir: 10.885_826_437_575_982,
            diffuse_nir: 20.063_182_822_663_31,
        };
        let multirank_potential_trial = vec![
            -5_900.0, -5_450.0, -4_300.0, -2_850.0, 0.68, 0.66, 296.2, 295.4, 295.6, 295.2,
            -5_900.0, -5_450.0, -4_300.0, -2_850.0, 0.68, 0.66, 295.5, 295.0, 295.6, 295.2, 295.8,
            0.011, 295.0, 291.5, 289.8,
        ];
        let multirank_potential =
            match solve_covered_column(&column, None, multirank_potential_trial.clone())
                .expect("multirank potential")
            {
                CoveredColumnSolveOutcome::Accepted(value) => value,
                CoveredColumnSolveOutcome::Rejected(failure) => {
                    panic!("multirank potential rejected: {failure:?}")
                }
            };
        assert_eq!(multirank_potential.root_water.len(), 8);
        let mut multirank_root_caps = BTreeMap::new();
        for source in &multirank_potential.root_water {
            let request_rate =
                source.request_kg_m2_stand_ground / (column.tile_fraction * column.interval_s);
            let authorization_rate = match (source.occupancy_id.as_str(), source.layer_id.as_str())
            {
                ("canopy-rank-0", "soil-1") => 5.449_439_753_166_751_5e-6,
                ("canopy-rank-0", "soil-2") => 2.003_239_473_339_951_3e-6,
                ("canopy-rank-1", "soil-1") => 4.035_383_488_425_889e-6,
                ("canopy-rank-1", "soil-2") => 1.452_434_951_361_456_6e-6,
                _ => 0.0,
            };
            multirank_root_caps.insert(
                (source.occupancy_id.clone(), source.layer_id.clone()),
                SourceWaterCap {
                    request_rate_kg_m2_tile_s: request_rate,
                    authorization_rate_kg_m2_tile_s: authorization_rate,
                },
            );
        }
        let multirank_caps = CoveredWaterCaps {
            root: multirank_root_caps,
            ground: SourceWaterCap {
                request_rate_kg_m2_tile_s: multirank_potential
                    .ground_water
                    .request_kg_m2_stand_ground
                    / (column.tile_fraction * column.interval_s),
                authorization_rate_kg_m2_tile_s: 1.433_503_758_902_920_5e-4,
            },
        };
        let multirank_capped_trial = multirank_potential_trial.clone();
        let multirank = execute_covered_potential_final(
            &column,
            multirank_potential_trial,
            &multirank_caps,
            multirank_capped_trial,
        )
        .expect("multirank potential/final transaction");
        assert_eq!(multirank.final_pass.evaluation.occupancies.len(), 2);
        assert!(multirank.final_pass.root_water.iter().all(|source| {
            source.finalized_use_kg_m2_stand_ground
                <= source.authorization_kg_m2_stand_ground.unwrap_or(0.0)
        }));
    }
}
