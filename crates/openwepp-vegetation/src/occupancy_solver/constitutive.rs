#![allow(
    clippy::float_cmp,
    clippy::many_single_char_names,
    clippy::module_name_repetitions,
    clippy::similar_names,
    clippy::struct_field_names,
    clippy::too_many_lines
)]
//! V3 Stage-A evaluator transcribed from the released independent oracle.

use std::collections::BTreeSet;

use openwepp_kernel_contract::SoilLayerId;
use openwepp_kernel_contract::{OccupancyId, TransactionId};
use serde::{Deserialize, Serialize};

use super::potential::{
    CappedLayerFluxEvaluation, StageAEvaluation, StageAEvaluator, StageASolution,
    StageASolveIdentity, StageAState, solve_uncapped_stage_a,
};
use crate::VegetationError;
use crate::diagnostics::{
    CoupledSolvePass, NormalizedResidual, NumericalFailureDiagnostics, SolveIdentity,
};
use crate::energy::{
    LATENT_HEAT_VAPORIZATION, STEFAN_BOLTZMANN, canopy_surface_friction_velocity,
    neutral_resistance, saturation_specific_humidity,
};
use crate::error::NumericalFailureCategory;
use crate::occupancy_state::OccupancyState;
use crate::photosynthesis::{FvcbInput, arrhenius, fvcb, peaked_response};

const R_GAS: f64 = 8.314_462_618_153_24;
const AIR_DENSITY_GAS_CONSTANT: f64 = 287.05;
const SPECIFIC_HEAT_AIR_J_KG_K: f64 = 1_004.64;
const BOUNDARY_CONDUCTANCE_COEFFICIENT_M_S_HALF: f64 = 0.01;
const OXYGEN_MOLE_FRACTION: f64 = 0.20;
const ELECTRON_QUANTUM_YIELD: f64 = 0.85;
const PAR_PHOTON_UMOL_PER_J: f64 = 4.6;
const ELECTRON_CURVATURE: f64 = 0.7;
const AC_AJ_CURVATURE: f64 = 0.98;
const AG_AP_CURVATURE: f64 = 0.95;
const ENERGY_MAX_ITERATIONS: u32 = 50;
const ENERGY_MAX_HALVINGS: u32 = 20;
const ENERGY_ATOL: f64 = 1.0e-6;
const ENERGY_RTOL: f64 = 1.0e-10;
const VAPOR_ATOL: f64 = 1.0e-12;
const VAPOR_RTOL: f64 = 1.0e-9;
const ENERGY_STEP_TOLERANCE_K: f64 = 1.0e-8;

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct V3PotentialCase {
    pub tile_fraction: f64,
    pub dt_s: f64,
    pub gas_energy: GasEnergyOperands,
    pub classes: LeafClasses,
    pub biochemical_parameters: BiochemicalParameters,
    pub parameters: HydraulicParameters,
    pub layers: Vec<LayerOperands>,
    pub surface_dimensions: SurfaceDimensions,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct ClassMaximumDemand {
    pub sun: f64,
    pub shade: f64,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct SurfaceDimensions {
    pub leaf_m: f64,
    pub wet_surface_m: f64,
    pub stem_m: f64,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct LeafClasses {
    pub sun: LeafClassOperands,
    pub shade: LeafClassOperands,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct LeafClassOperands {
    pub leaf_area: f64,
    pub absorbed_par_w_m2_leaf: f64,
    pub absorbed_shortwave_w_m2_tile: f64,
    pub vcmax25: f64,
    pub jmax25: f64,
    pub rd25: f64,
    pub temperature_start_k: f64,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct ReferenceWindOperands {
    pub kappa: f64,
    pub u_ref_m_s: f64,
    pub z_ref_m: f64,
    pub displacement_m: f64,
    pub z0m_m: f64,
    pub z0h_m: f64,
    pub z0q_m: f64,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct GasEnergyOperands {
    pub pressure_pa: f64,
    pub ca_pa: f64,
    pub derived_u_star_m_s: f64,
    pub gb_leaf_m_s: f64,
    pub gb_wet_m_s: f64,
    pub gb_stem_m_s: f64,
    pub g0_umol_m2_s: f64,
    pub medlyn_g1_kpa_sqrt: f64,
    pub cp_air_j_kg_k: f64,
    pub latent_heat_j_kg: f64,
    pub rdry_j_kg_k: f64,
    pub air_temperature_k: f64,
    pub air_specific_humidity_kg_kg: f64,
    pub reference_wind_operands: ReferenceWindOperands,
    pub rah_s_m: f64,
    pub raw_s_m: f64,
    pub leaf_emissivity: f64,
    pub wet_emissivity: f64,
    pub stem_emissivity: f64,
    pub longwave_down_w_m2: f64,
    pub longwave_up_w_m2: f64,
    pub stem_area: f64,
    pub stem_absorbed_shortwave_w_m2_tile: f64,
    pub wet_fraction: f64,
    pub canopy_liquid_kg_m2_tile: f64,
    pub dt_s: f64,
    pub wet_temperature_start_k: f64,
    pub stem_temperature_start_k: f64,
    pub canopy_air_temperature_start_k: f64,
    pub qcan_start_kg_kg: f64,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct BiochemicalParameters {
    pub kc25_pa: f64,
    pub ko25_pa: f64,
    pub gamma25_pa: f64,
    pub ha_vcmax_j_mol: f64,
    pub hd_vcmax_j_mol: f64,
    pub entropy_vcmax_j_mol_k: f64,
    pub ha_jmax_j_mol: f64,
    pub hd_jmax_j_mol: f64,
    pub entropy_jmax_j_mol_k: f64,
    pub ha_kc_j_mol: f64,
    pub ha_ko_j_mol: f64,
    pub ha_gamma_j_mol: f64,
    pub tp_vcmax_ratio: f64,
    pub oxygen_mole_fraction: f64,
    pub electron_quantum_yield: f64,
    pub par_photon_umol_per_j: f64,
    pub electron_curvature: f64,
    pub ac_aj_curvature: f64,
    pub ag_ap_curvature: f64,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct HydraulicParameters {
    pub k1a_max_s1: f64,
    pub k1b_max_s1: f64,
    pub k2_max: f64,
    pub height_m: f64,
    pub k3_max_m_s: f64,
    pub root_to_leaf_area: f64,
    pub p50_root: f64,
    pub p50_xylem: f64,
    pub p50_leaf: f64,
    pub ck: f64,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct LayerOperands {
    pub layer_id: String,
    pub soil_potential_mm: f64,
    pub gravity_head_mm: f64,
    pub root_fraction: f64,
    pub z3_m: f64,
    pub ksoil_m2_s: f64,
    pub dxroot_m: f64,
    pub accessible: bool,
    pub frozen: bool,
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct ClassGasState {
    pub transpiration_kg_m2_tile_s: f64,
    pub ci_pa: f64,
    pub leaf_temperature_k: f64,
    pub ci_iterations: u32,
    pub ci_bracket_pa: (f64, f64),
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct CanopyEnergyResult {
    pub sun: ClassGasState,
    pub shade: ClassGasState,
    pub wet_surface_temperature_k: f64,
    pub dry_stem_temperature_k: f64,
    pub canopy_air_temperature_k: f64,
    pub canopy_air_specific_humidity_kg_kg: f64,
    pub wet_actual_kg_m2_s: f64,
    pub wet_store_cap_active: bool,
    pub iterations: u32,
    pub backtracking_count: u32,
    pub temperature_step_k: Option<f64>,
    pub normalized_residuals: Vec<NormalizedResidual>,
    pub pivot_magnitude: Option<f64>,
    pub matrix_norm: Option<f64>,
}

#[derive(Clone, Debug)]
struct EnergyResidualDetail {
    residuals: [f64; 6],
    sun: ClassGasState,
    shade: ClassGasState,
    wet_actual: f64,
    wet_store_cap_active: bool,
    component_scales: [f64; 5],
    vapor_scale: f64,
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct ConstitutiveSolveContext {
    pub transaction_id: TransactionId,
    pub occupancy_id: OccupancyId,
    pub pass: CoupledSolvePass,
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct V3AcceptedStageA {
    pub outer: StageASolution,
    pub canopy: CanopyEnergyResult,
}

impl V3AcceptedStageA {
    pub(crate) fn occupancy_state(
        &self,
        beginning: &OccupancyState,
        canopy_liquid_kg_h2o_m2_tile_ground: f64,
    ) -> Result<OccupancyState, VegetationError> {
        if !canopy_liquid_kg_h2o_m2_tile_ground.is_finite()
            || canopy_liquid_kg_h2o_m2_tile_ground < 0.0
        {
            return Err(VegetationError::Domain("V3 accepted occupancy liquid"));
        }
        let candidate = OccupancyState {
            beta_hyd: self.outer.persisted_beta_hyd,
            canopy_air_specific_humidity_kg_kg: self.canopy.canopy_air_specific_humidity_kg_kg,
            canopy_air_temperature_k: self.canopy.canopy_air_temperature_k,
            canopy_liquid_kg_h2o_m2_tile_ground,
            dry_stem_temperature_k: self.canopy.dry_stem_temperature_k,
            last_accepted_transaction_id: beginning.last_accepted_transaction_id,
            root_node_potential_mm: self.outer.state.psi_root_mm,
            shade_ci_pa: self.canopy.shade.ci_pa,
            shade_leaf_potential_mm: self.outer.state.psi_shadeleaf_mm,
            shade_leaf_temperature_k: self.canopy.shade.leaf_temperature_k,
            stem_potential_mm: self.outer.state.psi_stem_mm,
            sun_ci_pa: self.canopy.sun.ci_pa,
            sun_leaf_potential_mm: self.outer.state.psi_sunleaf_mm,
            sun_leaf_temperature_k: self.canopy.sun.leaf_temperature_k,
            wet_surface_temperature_k: self.canopy.wet_surface_temperature_k,
        };
        candidate
            .validate(beginning.last_accepted_transaction_id)
            .map_err(|error| VegetationError::Schema(error.to_string()))?;
        Ok(candidate)
    }
}

#[derive(Clone, Debug)]
pub(crate) struct V3ConstitutiveEvaluator {
    case: V3PotentialCase,
    emax: ClassMaximumDemand,
    context: ConstitutiveSolveContext,
    next_evaluation_id: std::cell::Cell<u64>,
    evaluated_canopies: std::cell::RefCell<std::collections::BTreeMap<u64, CanopyEnergyResult>>,
    water_caps_kg_m2_s: Option<std::collections::BTreeMap<SoilLayerId, f64>>,
}

impl V3ConstitutiveEvaluator {
    pub(crate) fn new(
        case: V3PotentialCase,
        maximum_leaf_potentials_mm: (f64, f64),
        context: ConstitutiveSolveContext,
    ) -> Result<Self, VegetationError> {
        Self::new_with_caps(case, maximum_leaf_potentials_mm, context, None)
    }

    pub(crate) fn new_capped(
        case: V3PotentialCase,
        maximum_leaf_potentials_mm: (f64, f64),
        context: ConstitutiveSolveContext,
        water_caps_kg_m2_s: std::collections::BTreeMap<SoilLayerId, f64>,
    ) -> Result<Self, VegetationError> {
        Self::new_with_caps(
            case,
            maximum_leaf_potentials_mm,
            context,
            Some(water_caps_kg_m2_s),
        )
    }

    fn new_with_caps(
        case: V3PotentialCase,
        maximum_leaf_potentials_mm: (f64, f64),
        context: ConstitutiveSolveContext,
        water_caps_kg_m2_s: Option<std::collections::BTreeMap<SoilLayerId, f64>>,
    ) -> Result<Self, VegetationError> {
        validate_case(&case)?;
        validate_water_caps(&case, context.pass, water_caps_kg_m2_s.as_ref())?;
        let emax = if case.classes.sun.leaf_area + case.classes.shade.leaf_area == 0.0 {
            ClassMaximumDemand {
                sun: 0.0,
                shade: 0.0,
            }
        } else {
            let maximum =
                solve_canopy_energy(&case, (1.0, 1.0), maximum_leaf_potentials_mm, &context)?;
            ClassMaximumDemand {
                sun: maximum.sun.transpiration_kg_m2_tile_s,
                shade: maximum.shade.transpiration_kg_m2_tile_s,
            }
        };
        Ok(Self {
            case,
            emax,
            context,
            next_evaluation_id: std::cell::Cell::new(1),
            evaluated_canopies: std::cell::RefCell::new(std::collections::BTreeMap::new()),
            water_caps_kg_m2_s,
        })
    }

    pub(crate) fn maximum_demand(&self) -> &ClassMaximumDemand {
        &self.emax
    }

    #[cfg(test)]
    pub(super) fn with_released_singular_hydraulics(mut self) -> Self {
        self.case.parameters.k1a_max_s1 = 0.0;
        self.case.parameters.k1b_max_s1 = 0.0;
        self.case.parameters.k2_max = 0.0;
        for layer in &mut self.case.layers {
            layer.accessible = false;
        }
        self
    }

    fn retain_evaluated_canopy(&self, canopy: CanopyEnergyResult) -> Result<u64, VegetationError> {
        let evaluation_id = self.next_evaluation_id.get();
        let next = evaluation_id
            .checked_add(1)
            .ok_or(VegetationError::Domain("V3 evaluation identity overflow"))?;
        self.next_evaluation_id.set(next);
        self.evaluated_canopies
            .borrow_mut()
            .insert(evaluation_id, canopy);
        Ok(evaluation_id)
    }

    pub(crate) fn solve_uncapped(
        &self,
        identity: &StageASolveIdentity,
        initial: StageAState,
    ) -> Result<V3AcceptedStageA, VegetationError> {
        if self.context.pass != CoupledSolvePass::Potential
            || self.context.transaction_id != identity.transaction_id
            || self.context.occupancy_id != identity.occupancy_id
        {
            return Err(VegetationError::Domain("V3 constitutive solve identity"));
        }
        self.next_evaluation_id.set(1);
        self.evaluated_canopies.borrow_mut().clear();
        let outer = solve_uncapped_stage_a(identity, initial, self)?;
        let canopy = self
            .evaluated_canopies
            .borrow_mut()
            .remove(&outer.evaluation.evaluation_id)
            .ok_or(VegetationError::Coupled(
                "accepted nested canopy state unavailable",
            ))?;
        Ok(V3AcceptedStageA { outer, canopy })
    }

    pub(crate) fn solve_capped(
        &self,
        identity: &StageASolveIdentity,
        initial: StageAState,
    ) -> Result<V3AcceptedStageA, VegetationError> {
        if self.context.pass != CoupledSolvePass::Capped
            || self.context.transaction_id != identity.transaction_id
            || self.context.occupancy_id != identity.occupancy_id
            || self.water_caps_kg_m2_s.is_none()
        {
            return Err(VegetationError::Domain(
                "V3 capped constitutive solve identity",
            ));
        }
        self.next_evaluation_id.set(1);
        self.evaluated_canopies.borrow_mut().clear();
        let outer = solve_uncapped_stage_a(identity, initial, self)
            .map_err(Self::bind_cap_failure_diagnostics)?;
        let canopy = self
            .evaluated_canopies
            .borrow_mut()
            .remove(&outer.evaluation.evaluation_id)
            .ok_or(VegetationError::Coupled(
                "accepted capped nested canopy state unavailable",
            ))?;
        Ok(V3AcceptedStageA { outer, canopy })
    }

    #[cfg(test)]
    pub(super) fn solve_capped_with_limit(
        &self,
        identity: &StageASolveIdentity,
        initial: StageAState,
        max_iterations: u32,
    ) -> Result<V3AcceptedStageA, VegetationError> {
        if self.context.pass != CoupledSolvePass::Capped
            || self.context.transaction_id != identity.transaction_id
            || self.context.occupancy_id != identity.occupancy_id
            || self.water_caps_kg_m2_s.is_none()
        {
            return Err(VegetationError::Domain(
                "V5 capped constitutive solve identity",
            ));
        }
        self.next_evaluation_id.set(1);
        self.evaluated_canopies.borrow_mut().clear();
        let outer = super::potential::solve_uncapped_stage_a_bounded(
            identity,
            initial,
            self,
            max_iterations,
        )
        .map_err(Self::bind_cap_failure_diagnostics)?;
        let canopy = self
            .evaluated_canopies
            .borrow_mut()
            .remove(&outer.evaluation.evaluation_id)
            .ok_or(VegetationError::Coupled(
                "accepted capped nested canopy state unavailable",
            ))?;
        Ok(V3AcceptedStageA { outer, canopy })
    }

    fn bind_cap_failure_diagnostics(error: VegetationError) -> VegetationError {
        let VegetationError::NumericalFailure {
            category,
            mut diagnostics,
        } = error
        else {
            return error;
        };
        diagnostics.pass = CoupledSolvePass::Capped;
        VegetationError::NumericalFailure {
            category,
            diagnostics,
        }
    }

    fn active_water_caps(
        &self,
        psi_root_mm: f64,
        lai: f64,
        sai: f64,
    ) -> Result<Vec<SoilLayerId>, VegetationError> {
        let Some(caps) = &self.water_caps_kg_m2_s else {
            return Ok(Vec::new());
        };
        self.case
            .layers
            .iter()
            .filter_map(|layer| {
                let layer_id = SoilLayerId::try_new(layer.layer_id.clone()).ok()?;
                let cap = *caps.get(&layer_id)?;
                match layer_flux(&self.case, layer, psi_root_mm, lai, sai) {
                    Ok(law_flux) if cap <= law_flux => Some(Ok(layer_id)),
                    Ok(_) => None,
                    Err(error) => Some(Err(error)),
                }
            })
            .collect()
    }
}

fn validate_water_caps(
    case: &V3PotentialCase,
    pass: CoupledSolvePass,
    caps: Option<&std::collections::BTreeMap<SoilLayerId, f64>>,
) -> Result<(), VegetationError> {
    match (pass, caps) {
        (CoupledSolvePass::Potential, None) => return Ok(()),
        (CoupledSolvePass::Capped, Some(_)) => {}
        _ => return Err(VegetationError::Domain("V3 constitutive cap/pass identity")),
    }
    let caps = caps.ok_or(VegetationError::Domain("V3 constitutive cap identity"))?;
    let expected = case
        .layers
        .iter()
        .map(|layer| {
            SoilLayerId::try_new(layer.layer_id.clone())
                .map_err(|_| VegetationError::Domain("V3 constitutive layer identity"))
        })
        .collect::<Result<BTreeSet<_>, _>>()?;
    let actual = caps.keys().cloned().collect::<BTreeSet<_>>();
    if actual != expected || caps.values().any(|cap| !cap.is_finite() || *cap < 0.0) {
        return Err(VegetationError::Domain(
            "V3 constitutive water-cap identity",
        ));
    }
    Ok(())
}

impl StageAEvaluator for V3ConstitutiveEvaluator {
    fn evaluate(&self, state: StageAState) -> Result<StageAEvaluation, VegetationError> {
        self.evaluate_with_frozen_caps(state, None)
    }

    fn evaluate_jacobian_perturbation(
        &self,
        state: StageAState,
        unperturbed: &StageAEvaluation,
    ) -> Result<StageAEvaluation, VegetationError> {
        if self.water_caps_kg_m2_s.is_none() {
            return self.evaluate(state);
        }
        let frozen = unperturbed
            .capped_layer_fluxes
            .iter()
            .filter(|layer| layer.authorization_active_or_tie)
            .map(|layer| layer.layer_id.clone())
            .collect::<BTreeSet<_>>();
        self.evaluate_with_frozen_caps(state, Some(&frozen))
    }
}

impl V3ConstitutiveEvaluator {
    fn evaluate_with_frozen_caps(
        &self,
        state: StageAState,
        frozen_active_caps: Option<&BTreeSet<SoilLayerId>>,
    ) -> Result<StageAEvaluation, VegetationError> {
        if ![
            state.psi_sunleaf_mm,
            state.psi_shadeleaf_mm,
            state.psi_stem_mm,
            state.psi_root_mm,
            state.beta_sun,
            state.beta_shade,
        ]
        .iter()
        .all(|value| value.is_finite())
            || !(0.0..=1.0).contains(&state.beta_sun)
            || !(0.0..=1.0).contains(&state.beta_shade)
        {
            return Err(VegetationError::Domain("V3 constitutive Stage-A state"));
        }
        let p = &self.case.parameters;
        let sun_area = self.case.classes.sun.leaf_area;
        let shade_area = self.case.classes.shade.leaf_area;
        let sai = self.case.gas_energy.stem_area;
        let lai = sun_area + shade_area;
        if lai == 0.0 {
            let canopy = solve_canopy_energy(
                &self.case,
                (state.beta_sun, state.beta_shade),
                (state.psi_sunleaf_mm, state.psi_shadeleaf_mm),
                &self.context,
            )?;
            let evaluation_id = self.retain_evaluated_canopy(canopy)?;
            return zero_lai_evaluation(
                &self.case,
                evaluation_id,
                self.water_caps_kg_m2_s.as_ref(),
            );
        }
        let energy = solve_canopy_energy(
            &self.case,
            (state.beta_sun, state.beta_shade),
            (state.psi_sunleaf_mm, state.psi_shadeleaf_mm),
            &self.context,
        )?;
        let evaluation_id = self.retain_evaluated_canopy(energy.clone())?;
        let stem_vulnerability = vulnerability(state.psi_stem_mm, p.p50_xylem, p.ck)?;
        let q1_sun = p.k1a_max_s1
            * sun_area
            * stem_vulnerability
            * (state.psi_stem_mm - state.psi_sunleaf_mm);
        let q1_shade = p.k1b_max_s1
            * shade_area
            * stem_vulnerability
            * (state.psi_stem_mm - state.psi_shadeleaf_mm);
        let q2 = p.k2_max / p.height_m
            * vulnerability(state.psi_root_mm, p.p50_xylem, p.ck)?
            * sai
            * (state.psi_root_mm - state.psi_stem_mm - 1_000.0 * p.height_m);
        let mut q3 = Vec::with_capacity(self.case.layers.len());
        let mut capped_layer_fluxes = Vec::with_capacity(self.case.layers.len());
        for layer in &self.case.layers {
            let layer_id = SoilLayerId::try_new(layer.layer_id.clone())
                .map_err(|_| VegetationError::Domain("V3 constitutive layer identity"))?;
            let law_flux = layer_flux(&self.case, layer, state.psi_root_mm, lai, sai)?;
            if self.water_caps_kg_m2_s.is_some() && law_flux < 0.0 {
                return Err(VegetationError::Hydraulic(
                    "hydraulic redistribution unsupported",
                ));
            }
            let cap = self
                .water_caps_kg_m2_s
                .as_ref()
                .and_then(|caps| caps.get(&layer_id));
            let frozen_active = frozen_active_caps.map(|set| set.contains(&layer_id));
            let (flux, active) = cap.map_or((law_flux, false), |cap_rate| {
                select_capped_flux(law_flux, *cap_rate, frozen_active)
            });
            if let Some(cap_rate) = cap {
                capped_layer_fluxes.push(CappedLayerFluxEvaluation {
                    layer_id: layer_id.clone(),
                    q_law_kg_m2_s: law_flux,
                    cap_rate_kg_m2_s: *cap_rate,
                    q_final_kg_m2_s: flux,
                    authorization_active_or_tie: active,
                    soil_potential_mm: layer.soil_potential_mm,
                    gravity_head_mm: layer.gravity_head_mm,
                    root_fraction: layer.root_fraction,
                    z3_m: layer.z3_m,
                    ksoil_m2_s: layer.ksoil_m2_s,
                    dxroot_m: layer.dxroot_m,
                    accessible: layer.accessible,
                    frozen: layer.frozen,
                });
            }
            q3.push((layer_id, flux));
        }
        let gas_sun = energy.sun.transpiration_kg_m2_tile_s;
        let gas_shade = energy.shade.transpiration_kg_m2_tile_s;
        Ok(StageAEvaluation {
            evaluation_id,
            capped_system: self.water_caps_kg_m2_s.is_some(),
            emax_sun_kg_m2_s: self.emax.sun,
            emax_shade_kg_m2_s: self.emax.shade,
            gas_sun_kg_m2_s: gas_sun,
            gas_shade_kg_m2_s: gas_shade,
            vulnerability_demand_sun_kg_m2_s: self.emax.sun
                * vulnerability(state.psi_sunleaf_mm, p.p50_leaf, p.ck)?,
            vulnerability_demand_shade_kg_m2_s: self.emax.shade
                * vulnerability(state.psi_shadeleaf_mm, p.p50_leaf, p.ck)?,
            q1_sun_kg_m2_s: q1_sun,
            q1_shade_kg_m2_s: q1_shade,
            q2_kg_m2_s: q2,
            q3_kg_m2_s: q3,
            capped_layer_fluxes,
            active_water_caps: self.active_water_caps(state.psi_root_mm, lai, sai)?,
        })
    }
}

pub(super) fn select_capped_flux(
    q_law_kg_m2_s: f64,
    cap_rate_kg_m2_s: f64,
    frozen_active: Option<bool>,
) -> (f64, bool) {
    let active = frozen_active.unwrap_or(cap_rate_kg_m2_s <= q_law_kg_m2_s);
    (
        if active {
            cap_rate_kg_m2_s
        } else {
            q_law_kg_m2_s
        },
        active,
    )
}

fn zero_lai_evaluation(
    case: &V3PotentialCase,
    evaluation_id: u64,
    caps: Option<&std::collections::BTreeMap<SoilLayerId, f64>>,
) -> Result<StageAEvaluation, VegetationError> {
    let q3_kg_m2_s = case
        .layers
        .iter()
        .map(|layer| {
            SoilLayerId::try_new(layer.layer_id.clone())
                .map(|layer_id| (layer_id, 0.0))
                .map_err(|_| VegetationError::Domain("V3 constitutive layer identity"))
        })
        .collect::<Result<Vec<_>, _>>()?;
    let capped_layer_fluxes = caps
        .map(|caps| {
            q3_kg_m2_s
                .iter()
                .map(|(layer_id, _)| {
                    let cap_rate = *caps
                        .get(layer_id)
                        .ok_or(VegetationError::Domain("V5 zero-LAI cap identity"))?;
                    Ok(CappedLayerFluxEvaluation {
                        layer_id: layer_id.clone(),
                        q_law_kg_m2_s: 0.0,
                        cap_rate_kg_m2_s: cap_rate,
                        q_final_kg_m2_s: 0.0,
                        authorization_active_or_tie: cap_rate <= 0.0,
                        soil_potential_mm: 0.0,
                        gravity_head_mm: 0.0,
                        root_fraction: 0.0,
                        z3_m: 0.0,
                        ksoil_m2_s: 0.0,
                        dxroot_m: 0.0,
                        accessible: false,
                        frozen: false,
                    })
                })
                .collect::<Result<Vec<_>, VegetationError>>()
        })
        .transpose()?
        .unwrap_or_default();
    let active_water_caps = capped_layer_fluxes
        .iter()
        .filter(|layer| layer.authorization_active_or_tie)
        .map(|layer| layer.layer_id.clone())
        .collect();
    Ok(StageAEvaluation {
        evaluation_id,
        capped_system: caps.is_some(),
        emax_sun_kg_m2_s: 0.0,
        emax_shade_kg_m2_s: 0.0,
        gas_sun_kg_m2_s: 0.0,
        gas_shade_kg_m2_s: 0.0,
        vulnerability_demand_sun_kg_m2_s: 0.0,
        vulnerability_demand_shade_kg_m2_s: 0.0,
        q1_sun_kg_m2_s: 0.0,
        q1_shade_kg_m2_s: 0.0,
        q2_kg_m2_s: 0.0,
        q3_kg_m2_s,
        capped_layer_fluxes,
        active_water_caps,
    })
}

pub(super) fn solve_canopy_energy(
    case: &V3PotentialCase,
    betas: (f64, f64),
    leaf_potentials_mm: (f64, f64),
    context: &ConstitutiveSolveContext,
) -> Result<CanopyEnergyResult, VegetationError> {
    solve_canopy_energy_with_limit(
        case,
        betas,
        leaf_potentials_mm,
        context,
        ENERGY_MAX_ITERATIONS,
    )
}

pub(super) fn solve_canopy_energy_with_limit(
    case: &V3PotentialCase,
    betas: (f64, f64),
    leaf_potentials_mm: (f64, f64),
    context: &ConstitutiveSolveContext,
    max_iterations: u32,
) -> Result<CanopyEnergyResult, VegetationError> {
    if !(0.0..=1.0).contains(&betas.0) || !(0.0..=1.0).contains(&betas.1) {
        return Err(VegetationError::Domain("V3 class beta_hyd"));
    }
    if !leaf_potentials_mm.0.is_finite() || !leaf_potentials_mm.1.is_finite() {
        return Err(VegetationError::Domain("V3 leaf potential input"));
    }
    let forcing = &case.gas_energy;
    let mut x = [
        case.classes.sun.temperature_start_k,
        case.classes.shade.temperature_start_k,
        forcing.wet_temperature_start_k,
        forcing.stem_temperature_start_k,
        forcing.canopy_air_temperature_start_k,
        forcing.qcan_start_kg_kg,
    ];
    let mut backtracking = 0;
    let mut last_temperature_step = None;
    let mut last_detail = energy_residual(case, betas, &x, context)
        .map_err(|error| wrap_energy_error(context, error, 0, None, 0, None, None))?;
    let mut last_pivot = None;
    let mut last_matrix_norm = None;
    for iteration in 0..=max_iterations {
        let normalized = normalized_energy(&last_detail);
        let norm = infinity_norm(&normalized);
        if norm <= 1.0 && last_temperature_step.is_none_or(|step| step <= ENERGY_STEP_TOLERANCE_K) {
            return Ok(energy_result(
                x,
                last_detail,
                iteration,
                backtracking,
                last_temperature_step,
                last_pivot,
                last_matrix_norm,
            ));
        }
        if iteration == max_iterations {
            return Err(energy_failure(
                context,
                NumericalFailureCategory::IterationLimit,
                iteration,
                &last_detail,
                last_temperature_step,
                backtracking,
                last_pivot,
                last_matrix_norm,
            ));
        }
        let mut jacobian = [[0.0; 6]; 6];
        let unit_scales = [1.0, 1.0, 1.0, 1.0, 1.0, 1.0e-3];
        for column in 0..6 {
            let step = f64::EPSILON.sqrt() * x[column].abs().max(unit_scales[column]);
            let mut plus = x;
            let mut minus = x;
            plus[column] += step;
            minus[column] -= step;
            let rp = energy_residual(case, betas, &plus, context)
                .map_err(|error| {
                    wrap_energy_error(
                        context,
                        error,
                        iteration,
                        last_temperature_step,
                        backtracking,
                        last_pivot,
                        last_matrix_norm,
                    )
                })?
                .residuals;
            let rm = energy_residual(case, betas, &minus, context)
                .map_err(|error| {
                    wrap_energy_error(
                        context,
                        error,
                        iteration,
                        last_temperature_step,
                        backtracking,
                        last_pivot,
                        last_matrix_norm,
                    )
                })?
                .residuals;
            for row in 0..6 {
                jacobian[row][column] = (rp[row] - rm[row]) / (2.0 * step);
            }
        }
        let (delta, pivot, matrix_norm) = solve_linear(jacobian, last_detail.residuals.map(|v| -v))
            .map_err(|failure| {
                energy_failure(
                    context,
                    NumericalFailureCategory::SingularPivot,
                    iteration,
                    &last_detail,
                    last_temperature_step,
                    backtracking,
                    Some(failure.pivot_magnitude),
                    Some(failure.matrix_norm),
                )
            })?;
        last_pivot = Some(pivot);
        last_matrix_norm = Some(matrix_norm);
        let full_temperature_step = delta[..5].iter().copied().map(f64::abs).fold(0.0, f64::max);
        if norm <= 1.0 && full_temperature_step <= ENERGY_STEP_TOLERANCE_K {
            last_temperature_step = Some(full_temperature_step);
            return Ok(energy_result(
                x,
                last_detail,
                iteration,
                backtracking,
                last_temperature_step,
                last_pivot,
                last_matrix_norm,
            ));
        }
        let mut accepted = None;
        for half in 0..=ENERGY_MAX_HALVINGS {
            let exponent = i32::try_from(half)
                .map_err(|_| VegetationError::Domain("V3 energy backtracking"))?;
            let factor = 2.0_f64.powi(-exponent);
            let mut trial = x;
            for index in 0..6 {
                trial[index] += factor * delta[index];
            }
            let detail = match energy_residual(case, betas, &trial, context) {
                Ok(detail) => detail,
                Err(error @ VegetationError::NumericalFailure { .. }) => {
                    return Err(error);
                }
                Err(_) => {
                    backtracking += 1;
                    continue;
                }
            };
            if infinity_norm(&normalized_energy(&detail)) < norm {
                accepted = Some((trial, detail, factor));
                break;
            }
            backtracking += 1;
        }
        let Some((next, detail, factor)) = accepted else {
            return Err(energy_failure(
                context,
                NumericalFailureCategory::BacktrackingLimit,
                iteration,
                &last_detail,
                Some(full_temperature_step),
                backtracking,
                last_pivot,
                last_matrix_norm,
            ));
        };
        last_temperature_step = Some(full_temperature_step * factor);
        x = next;
        last_detail = detail;
    }
    unreachable!("bounded V3 canopy-energy loop")
}

fn energy_result(
    x: [f64; 6],
    detail: EnergyResidualDetail,
    iterations: u32,
    backtracking_count: u32,
    temperature_step_k: Option<f64>,
    pivot_magnitude: Option<f64>,
    matrix_norm: Option<f64>,
) -> CanopyEnergyResult {
    let normalized_residuals = labeled_energy(&normalized_energy(&detail));
    CanopyEnergyResult {
        sun: detail.sun,
        shade: detail.shade,
        wet_surface_temperature_k: x[2],
        dry_stem_temperature_k: x[3],
        canopy_air_temperature_k: x[4],
        canopy_air_specific_humidity_kg_kg: x[5],
        wet_actual_kg_m2_s: detail.wet_actual,
        wet_store_cap_active: detail.wet_store_cap_active,
        iterations,
        backtracking_count,
        temperature_step_k,
        normalized_residuals,
        pivot_magnitude,
        matrix_norm,
    }
}

fn energy_residual(
    case: &V3PotentialCase,
    betas: (f64, f64),
    x: &[f64; 6],
    context: &ConstitutiveSolveContext,
) -> Result<EnergyResidualDetail, VegetationError> {
    if x[..5]
        .iter()
        .any(|value| !(273.15..=373.15).contains(value))
        || !x[5].is_finite()
        || x[5] < 0.0
    {
        return Err(VegetationError::Domain("V3 canopy energy state"));
    }
    let f = &case.gas_energy;
    let pressure = f.pressure_pa;
    let tcan = x[4];
    let qcan = x[5];
    let rho = pressure / (f.rdry_j_kg_k * tcan);
    let wet_fraction = f.wet_fraction;
    let dry_sun_area = case.classes.sun.leaf_area * (1.0 - wet_fraction);
    let dry_shade_area = case.classes.shade.leaf_area * (1.0 - wet_fraction);
    let wet_leaf_area = wet_fraction * (case.classes.sun.leaf_area + case.classes.shade.leaf_area);
    let wet_stem_area = wet_fraction * f.stem_area;
    let wet_area = wet_leaf_area + wet_stem_area;
    let dry_stem_area = (1.0 - wet_fraction) * f.stem_area;
    let sun = solve_class(
        case,
        &case.classes.sun,
        x[0],
        qcan,
        betas.0,
        context,
        SolveIdentity::SunCi,
    )?;
    let shade = solve_class(
        case,
        &case.classes.shade,
        x[1],
        qcan,
        betas.1,
        context,
        SolveIdentity::ShadeCi,
    )?;
    let sun_flux = if dry_sun_area == 0.0 {
        0.0
    } else {
        rho * (saturation_specific_humidity(x[0], pressure)? - qcan)
            / (1.0 / f.gb_leaf_m_s + sun.rs_s_m)
            * dry_sun_area
    };
    let shade_flux = if dry_shade_area == 0.0 {
        0.0
    } else {
        rho * (saturation_specific_humidity(x[1], pressure)? - qcan)
            / (1.0 / f.gb_leaf_m_s + shade.rs_s_m)
            * dry_shade_area
    };
    let leaf_residual =
        |temperature: f64, area: f64, absorbed: f64, transpiration: f64| -> (f64, f64) {
            let sw = absorbed * (1.0 - wet_fraction);
            let lw = f.leaf_emissivity
                * area
                * (f.longwave_down_w_m2 + f.longwave_up_w_m2
                    - 2.0 * STEFAN_BOLTZMANN * temperature.powi(4));
            let sensible = rho * f.cp_air_j_kg_k * f.gb_leaf_m_s * area * (temperature - tcan);
            let residual = sw + lw - sensible - f.latent_heat_j_kg * transpiration;
            let scale =
                (sw.abs() + lw.abs() + sensible.abs() + (f.latent_heat_j_kg * transpiration).abs())
                    .max(1.0);
            (residual, scale)
        };
    let (sun_residual, sun_scale) = if dry_sun_area == 0.0 {
        (x[0] - tcan, 1.0)
    } else {
        leaf_residual(
            x[0],
            dry_sun_area,
            case.classes.sun.absorbed_shortwave_w_m2_tile,
            sun_flux,
        )
    };
    let (shade_residual, shade_scale) = if dry_shade_area == 0.0 {
        (x[1] - tcan, 1.0)
    } else {
        leaf_residual(
            x[1],
            dry_shade_area,
            case.classes.shade.absorbed_shortwave_w_m2_tile,
            shade_flux,
        )
    };
    let (wet_actual, wet_cap_active, wet_residual, wet_scale) = if wet_area == 0.0 {
        (0.0, false, x[2] - tcan, 1.0)
    } else {
        let qsat_wet = saturation_specific_humidity(x[2], pressure)?;
        let wet_potential = rho * f.gb_wet_m_s * (qsat_wet - qcan) * wet_area;
        let store_cap = f.canopy_liquid_kg_m2_tile / f.dt_s;
        let wet_actual = if wet_potential >= 0.0 {
            wet_potential.min(store_cap)
        } else {
            wet_potential
        };
        let wet_cap_active = wet_potential > store_cap;
        let wet_sw = wet_fraction
            * (case.classes.sun.absorbed_shortwave_w_m2_tile
                + case.classes.shade.absorbed_shortwave_w_m2_tile
                + f.stem_absorbed_shortwave_w_m2_tile);
        let wet_lw = f.wet_emissivity
            * wet_area
            * (f.longwave_down_w_m2 + f.longwave_up_w_m2 - 2.0 * STEFAN_BOLTZMANN * x[2].powi(4));
        let wet_h = rho * f.cp_air_j_kg_k * f.gb_wet_m_s * wet_area * (x[2] - tcan);
        let wet_residual = wet_sw + wet_lw - wet_h - f.latent_heat_j_kg * wet_actual;
        let wet_scale =
            (wet_sw.abs() + wet_lw.abs() + wet_h.abs() + (f.latent_heat_j_kg * wet_actual).abs())
                .max(1.0);
        (wet_actual, wet_cap_active, wet_residual, wet_scale)
    };
    let stem_sw = (1.0 - wet_fraction) * f.stem_absorbed_shortwave_w_m2_tile;
    let stem_lw = f.stem_emissivity
        * dry_stem_area
        * (f.longwave_down_w_m2 + f.longwave_up_w_m2 - 2.0 * STEFAN_BOLTZMANN * x[3].powi(4));
    let stem_h = rho * f.cp_air_j_kg_k * f.gb_stem_m_s * dry_stem_area * (x[3] - tcan);
    let (stem_residual, stem_scale) = if dry_stem_area == 0.0 {
        (x[3] - tcan, 1.0)
    } else {
        (
            stem_sw + stem_lw - stem_h,
            (stem_sw.abs() + stem_lw.abs() + stem_h.abs()).max(1.0),
        )
    };
    let heat_terms = (tcan - f.air_temperature_k) / f.rah_s_m
        - f.gb_leaf_m_s * dry_sun_area * (x[0] - tcan)
        - f.gb_leaf_m_s * dry_shade_area * (x[1] - tcan)
        - f.gb_wet_m_s * wet_area * (x[2] - tcan)
        - f.gb_stem_m_s * dry_stem_area * (x[3] - tcan);
    let heat_balance = rho * f.cp_air_j_kg_k * heat_terms;
    let heat_scale = (rho * f.cp_air_j_kg_k * (tcan - f.air_temperature_k) / f.rah_s_m).abs()
        + (rho * f.cp_air_j_kg_k * f.gb_leaf_m_s * dry_sun_area * (x[0] - tcan)).abs()
        + (rho * f.cp_air_j_kg_k * f.gb_leaf_m_s * dry_shade_area * (x[1] - tcan)).abs()
        + (rho * f.cp_air_j_kg_k * f.gb_wet_m_s * wet_area * (x[2] - tcan)).abs()
        + (rho * f.cp_air_j_kg_k * f.gb_stem_m_s * dry_stem_area * (x[3] - tcan)).abs();
    let vapor_atmosphere = rho * (qcan - f.air_specific_humidity_kg_kg) / f.raw_s_m;
    let vapor_balance = vapor_atmosphere - sun_flux - shade_flux - wet_actual;
    let vapor_scale = vapor_atmosphere
        .abs()
        .max(wet_actual.abs())
        .max(sun_flux.abs())
        .max(shade_flux.abs())
        .max(1.0e-12);
    Ok(EnergyResidualDetail {
        residuals: [
            sun_residual,
            shade_residual,
            wet_residual,
            stem_residual,
            heat_balance,
            vapor_balance,
        ],
        sun: ClassGasState {
            transpiration_kg_m2_tile_s: sun_flux,
            ci_pa: sun.ci_pa,
            leaf_temperature_k: x[0],
            ci_iterations: sun.iterations,
            ci_bracket_pa: sun.bracket,
        },
        shade: ClassGasState {
            transpiration_kg_m2_tile_s: shade_flux,
            ci_pa: shade.ci_pa,
            leaf_temperature_k: x[1],
            ci_iterations: shade.iterations,
            ci_bracket_pa: shade.bracket,
        },
        wet_actual,
        wet_store_cap_active: wet_cap_active,
        component_scales: [
            sun_scale,
            shade_scale,
            wet_scale,
            stem_scale,
            heat_scale.max(1.0),
        ],
        vapor_scale,
    })
}

#[derive(Clone, Copy, Debug)]
pub(super) struct SolvedClass {
    pub(super) ci_pa: f64,
    pub(super) rs_s_m: f64,
    pub(super) iterations: u32,
    pub(super) bracket: (f64, f64),
}

fn solve_class(
    case: &V3PotentialCase,
    class: &LeafClassOperands,
    temperature_k: f64,
    qcan: f64,
    beta: f64,
    context: &ConstitutiveSolveContext,
    solve_identity: SolveIdentity,
) -> Result<SolvedClass, VegetationError> {
    solve_class_inner(
        case,
        class,
        temperature_k,
        qcan,
        beta,
        context,
        solve_identity,
    )
    .map_err(|error| {
        if matches!(error, VegetationError::NumericalFailure { .. }) {
            error
        } else {
            numerical_failure(
                context,
                NumericalFailureCategory::Domain,
                solve_identity,
                0,
                Vec::new(),
                None,
                0,
                None,
                None,
                None,
            )
        }
    })
}

fn solve_class_inner(
    case: &V3PotentialCase,
    class: &LeafClassOperands,
    temperature_k: f64,
    qcan: f64,
    beta: f64,
    context: &ConstitutiveSolveContext,
    solve_identity: SolveIdentity,
) -> Result<SolvedClass, VegetationError> {
    let f = &case.gas_energy;
    if class.leaf_area == 0.0 {
        return Ok(SolvedClass {
            ci_pa: f.ca_pa,
            rs_s_m: 0.0,
            iterations: 0,
            bracket: (f.ca_pa, f.ca_pa),
        });
    }
    let b = &case.biochemical_parameters;
    let pressure = f.pressure_pa;
    let qsat = saturation_specific_humidity(temperature_k, pressure)?;
    let es_leaf = qsat * pressure / (0.622 + 0.378 * qsat);
    let e_can = qcan * pressure / (0.622 + 0.378 * qcan);
    let vpd_kpa = (es_leaf - e_can) / 1_000.0;
    if vpd_kpa <= 0.0 || !vpd_kpa.is_finite() {
        return Err(VegetationError::Domain("V3 solved surface VPD"));
    }
    let vcmax_factor = peaked_response(
        temperature_k,
        b.ha_vcmax_j_mol,
        b.hd_vcmax_j_mol,
        b.entropy_vcmax_j_mol_k,
    )?;
    let jmax_factor = peaked_response(
        temperature_k,
        b.ha_jmax_j_mol,
        b.hd_jmax_j_mol,
        b.entropy_jmax_j_mol_k,
    )?;
    let vcmax = class.vcmax25 * vcmax_factor;
    let jmax = class.jmax25 * jmax_factor;
    let kc = b.kc25_pa * arrhenius(temperature_k, b.ha_kc_j_mol)?;
    let ko = b.ko25_pa * arrhenius(temperature_k, b.ha_ko_j_mol)?;
    let gamma = b.gamma25_pa * arrhenius(temperature_k, b.ha_gamma_j_mol)?;
    let tp = b.tp_vcmax_ratio * class.vcmax25 * vcmax_factor;
    let rd = class.rd25 * peaked_response(temperature_k, 46_390.0, 150_650.0, 490.0)?;
    let rb = 1.0 / f.gb_leaf_m_s;
    let residual = |ci: f64| -> Result<(f64, SolvedClass), VegetationError> {
        let photo = fvcb(FvcbInput {
            ci_pa: ci,
            oi_pa: b.oxygen_mole_fraction * pressure,
            gamma_pa: gamma,
            kc_pa: kc,
            ko_pa: ko,
            vcmax,
            jmax,
            tp,
            rd,
            par_abs: class.absorbed_par_w_m2_leaf,
        })?;
        let cs = f.ca_pa - 1.4 * rb * R_GAS * temperature_k * photo.an * 1.0e-6;
        if cs <= 0.0 || !cs.is_finite() {
            return Err(VegetationError::Domain("V3 surface carbon dioxide"));
        }
        let gs_potential = if photo.an <= 0.0 {
            f.g0_umol_m2_s
        } else {
            f.g0_umol_m2_s
                + 1.6 * (1.0 + f.medlyn_g1_kpa_sqrt / vpd_kpa.sqrt()) * photo.an / (cs / pressure)
        };
        let gs = f.g0_umol_m2_s + beta * (gs_potential - f.g0_umol_m2_s);
        let gs_m_s = gs * 1.0e-6 * R_GAS * temperature_k / pressure;
        if gs_m_s <= 0.0 || !gs_m_s.is_finite() {
            return Err(VegetationError::Domain("V3 stomatal conductance"));
        }
        let rs = 1.0 / gs_m_s;
        let predicted = f.ca_pa - (1.4 * rb + 1.6 * rs) * R_GAS * temperature_k * photo.an * 1.0e-6;
        Ok((
            ci - predicted,
            SolvedClass {
                ci_pa: ci,
                rs_s_m: rs,
                iterations: 0,
                bracket: (gamma, f.ca_pa),
            },
        ))
    };
    brent_dekker_class(residual, gamma, f.ca_pa, 64, context, solve_identity)
}

pub(super) fn brent_dekker_class<F>(
    mut function: F,
    low: f64,
    high: f64,
    max_evaluations: u32,
    context: &ConstitutiveSolveContext,
    solve_identity: SolveIdentity,
) -> Result<SolvedClass, VegetationError>
where
    F: FnMut(f64) -> Result<(f64, SolvedClass), VegetationError>,
{
    let mut a = low;
    let mut b = high;
    let (mut fa, _) = function(a).map_err(|error| {
        ci_failure(
            context,
            NumericalFailureCategory::Domain,
            solve_identity,
            1,
            Vec::new(),
            (a, b),
            error,
        )
    })?;
    let (mut fb, high_state) = function(b).map_err(|error| {
        ci_failure(
            context,
            NumericalFailureCategory::Domain,
            solve_identity,
            2,
            Vec::new(),
            (a, b),
            error,
        )
    })?;
    if !fa.is_finite() || !fb.is_finite() {
        return Err(ci_failure(
            context,
            NumericalFailureCategory::Domain,
            solve_identity,
            2,
            Vec::new(),
            (a, b),
            VegetationError::Domain("V3 ci nonfinite domain"),
        ));
    }
    if fa == 0.0 {
        let mut result = function(a)?.1;
        result.iterations = 2;
        result.bracket = (a, b);
        return Ok(result);
    }
    if fb == 0.0 {
        let mut result = high_state;
        result.iterations = 2;
        result.bracket = (a, b);
        return Ok(result);
    }
    if fa * fb > 0.0 {
        return Err(ci_failure(
            context,
            NumericalFailureCategory::BracketFailure,
            solve_identity,
            2,
            vec![fa / 1.0e-8, fb / 1.0e-8],
            (a, b),
            VegetationError::CiNonConvergence,
        ));
    }
    let mut c = a;
    let mut fc = fa;
    let mut d = b - a;
    let mut mflag = true;
    for evaluation in 3..=max_evaluations {
        let mut s = if fa != fc && fb != fc {
            a * fb * fc / ((fa - fb) * (fa - fc))
                + b * fa * fc / ((fb - fa) * (fb - fc))
                + c * fa * fb / ((fc - fa) * (fc - fb))
        } else {
            b - fb * (b - a) / (fb - fa)
        };
        let left = ((3.0 * a + b) / 4.0).min(b);
        let right = ((3.0 * a + b) / 4.0).max(b);
        let conditions = [
            !(left < s && s < right),
            mflag && (s - b).abs() >= (b - c).abs() / 2.0,
            !mflag && (s - b).abs() >= (c - d).abs() / 2.0,
            mflag && (b - c).abs() < 1.0e-6,
            !mflag && (c - d).abs() < 1.0e-6,
        ];
        if conditions.into_iter().any(|condition| condition) {
            s = 0.5 * (a + b);
            mflag = true;
        } else {
            mflag = false;
        }
        let (fs, _) = function(s).map_err(|error| {
            ci_failure(
                context,
                NumericalFailureCategory::Domain,
                solve_identity,
                evaluation,
                Vec::new(),
                (a.min(b), a.max(b)),
                error,
            )
        })?;
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
            let mut result = function(b)?.1;
            result.iterations = evaluation;
            result.bracket = (a.min(b), a.max(b));
            return Ok(result);
        }
    }
    Err(ci_failure(
        context,
        NumericalFailureCategory::IterationLimit,
        solve_identity,
        max_evaluations,
        vec![fa / 1.0e-8, fb / 1.0e-8],
        (a.min(b), a.max(b)),
        VegetationError::CiNonConvergence,
    ))
}

fn normalized_energy(detail: &EnergyResidualDetail) -> [f64; 6] {
    let mut normalized = [0.0; 6];
    for (index, value) in normalized[..5].iter_mut().enumerate() {
        *value =
            detail.residuals[index] / (ENERGY_ATOL + ENERGY_RTOL * detail.component_scales[index]);
    }
    normalized[5] = detail.residuals[5] / (VAPOR_ATOL + VAPOR_RTOL * detail.vapor_scale);
    normalized
}

pub(super) const ENERGY_RESIDUAL_IDENTITIES: [&str; 6] = [
    "sun_leaf_energy",
    "shade_leaf_energy",
    "wet_surface_energy",
    "dry_stem_energy",
    "canopy_air_heat",
    "canopy_air_vapor",
];

fn labeled_energy(values: &[f64; 6]) -> Vec<NormalizedResidual> {
    ENERGY_RESIDUAL_IDENTITIES
        .iter()
        .zip(values)
        .map(|(identity, value)| NormalizedResidual {
            identity: (*identity).into(),
            value: *value,
        })
        .collect()
}

#[allow(clippy::too_many_arguments)]
fn energy_failure(
    context: &ConstitutiveSolveContext,
    category: NumericalFailureCategory,
    iterations: u32,
    detail: &EnergyResidualDetail,
    step_norm: Option<f64>,
    backtracking_count: u32,
    pivot_magnitude: Option<f64>,
    matrix_norm: Option<f64>,
) -> VegetationError {
    numerical_failure(
        context,
        category,
        SolveIdentity::CanopyEnergy,
        iterations,
        labeled_energy(&normalized_energy(detail)),
        step_norm,
        backtracking_count,
        None,
        pivot_magnitude,
        matrix_norm,
    )
}

#[allow(clippy::too_many_arguments)]
fn wrap_energy_error(
    context: &ConstitutiveSolveContext,
    error: VegetationError,
    iterations: u32,
    step_norm: Option<f64>,
    backtracking_count: u32,
    pivot_magnitude: Option<f64>,
    matrix_norm: Option<f64>,
) -> VegetationError {
    if matches!(error, VegetationError::NumericalFailure { .. }) {
        error
    } else {
        numerical_failure(
            context,
            NumericalFailureCategory::Domain,
            SolveIdentity::CanopyEnergy,
            iterations,
            Vec::new(),
            step_norm,
            backtracking_count,
            None,
            pivot_magnitude,
            matrix_norm,
        )
    }
}

fn ci_failure(
    context: &ConstitutiveSolveContext,
    category: NumericalFailureCategory,
    solve: SolveIdentity,
    iterations: u32,
    residual_values: Vec<f64>,
    bracket: (f64, f64),
    source: VegetationError,
) -> VegetationError {
    if matches!(source, VegetationError::NumericalFailure { .. }) {
        return source;
    }
    let identities: &[&str] = if residual_values.len() <= 1 {
        match solve {
            SolveIdentity::SunCi => &["sun_ci"],
            SolveIdentity::ShadeCi => &["shade_ci"],
            _ => &["ci"],
        }
    } else {
        match solve {
            SolveIdentity::SunCi => &["sun_ci_bracket_low", "sun_ci_bracket_high"],
            SolveIdentity::ShadeCi => &["shade_ci_bracket_low", "shade_ci_bracket_high"],
            _ => &["ci_bracket_low", "ci_bracket_high"],
        }
    };
    numerical_failure(
        context,
        category,
        solve,
        iterations,
        identities
            .iter()
            .zip(residual_values)
            .map(|(identity, value)| NormalizedResidual {
                identity: (*identity).into(),
                value,
            })
            .collect(),
        None,
        0,
        Some(bracket),
        None,
        None,
    )
}

#[allow(clippy::too_many_arguments)]
fn numerical_failure(
    context: &ConstitutiveSolveContext,
    category: NumericalFailureCategory,
    solve: SolveIdentity,
    iterations: u32,
    residual_norms: Vec<NormalizedResidual>,
    step_norm: Option<f64>,
    backtracking_count: u32,
    bracket: Option<(f64, f64)>,
    pivot_magnitude: Option<f64>,
    matrix_norm: Option<f64>,
) -> VegetationError {
    let diagnostics = NumericalFailureDiagnostics {
        model_definition_sha256: crate::MODEL_SHA256.into(),
        transaction_id: context.transaction_id,
        occupancy_id: context.occupancy_id.clone(),
        pass: context.pass,
        solve,
        iterations,
        residual_norms,
        step_norm,
        backtracking_count,
        active_bounds: Vec::new(),
        active_water_caps: Vec::new(),
        bracket,
        pivot_magnitude,
        matrix_norm,
        capped_operands: None,
        fixed_authorization_identity: None,
    };
    debug_assert!(diagnostics.validate().is_ok());
    VegetationError::NumericalFailure {
        category,
        diagnostics: Box::new(diagnostics),
    }
}

fn infinity_norm(values: &[f64; 6]) -> f64 {
    values.iter().copied().map(f64::abs).fold(0.0, f64::max)
}

#[derive(Clone, Copy, Debug)]
pub(super) struct LinearSolveFailure {
    pub(super) pivot_magnitude: f64,
    pub(super) matrix_norm: f64,
}

pub(super) fn solve_linear(
    mut matrix: [[f64; 6]; 6],
    mut rhs: [f64; 6],
) -> Result<([f64; 6], f64, f64), LinearSolveFailure> {
    let matrix_norm = matrix
        .iter()
        .map(|row| row.iter().copied().map(f64::abs).sum::<f64>())
        .fold(0.0, f64::max);
    if !matrix_norm.is_finite() || matrix_norm <= 0.0 {
        return Err(LinearSolveFailure {
            pivot_magnitude: 0.0,
            matrix_norm: 0.0,
        });
    }
    let threshold = 64.0 * f64::EPSILON * matrix_norm;
    let mut minimum_pivot = f64::INFINITY;
    for column in 0..6 {
        let pivot_row = (column..6)
            .max_by(|left, right| {
                matrix[*left][column]
                    .abs()
                    .total_cmp(&matrix[*right][column].abs())
            })
            .ok_or(LinearSolveFailure {
                pivot_magnitude: 0.0,
                matrix_norm,
            })?;
        let pivot = matrix[pivot_row][column].abs();
        minimum_pivot = minimum_pivot.min(pivot);
        if !pivot.is_finite() || pivot <= threshold {
            return Err(LinearSolveFailure {
                pivot_magnitude: pivot,
                matrix_norm,
            });
        }
        matrix.swap(column, pivot_row);
        rhs.swap(column, pivot_row);
        let pivot_values = matrix[column];
        for row in column + 1..6 {
            let ratio = matrix[row][column] / matrix[column][column];
            for (value, pivot_value) in matrix[row][column..]
                .iter_mut()
                .zip(&pivot_values[column..])
            {
                *value -= ratio * pivot_value;
            }
            rhs[row] -= ratio * rhs[column];
        }
    }
    let mut solution = [0.0; 6];
    for row in (0..6).rev() {
        let tail = (row + 1..6)
            .map(|column| matrix[row][column] * solution[column])
            .sum::<f64>();
        solution[row] = (rhs[row] - tail) / matrix[row][row];
    }
    if solution.iter().any(|value| !value.is_finite()) {
        return Err(LinearSolveFailure {
            pivot_magnitude: minimum_pivot,
            matrix_norm,
        });
    }
    Ok((solution, minimum_pivot, matrix_norm))
}

fn vulnerability(potential_mm: f64, p50_mm: f64, exponent: f64) -> Result<f64, VegetationError> {
    if ![potential_mm, p50_mm, exponent]
        .iter()
        .all(|value| value.is_finite())
        || p50_mm >= 0.0
        || exponent <= 0.0
    {
        return Err(VegetationError::Domain("V3 vulnerability"));
    }
    let value = 2.0_f64.powf(-(potential_mm / p50_mm).powf(exponent));
    value
        .is_finite()
        .then_some(value)
        .ok_or(VegetationError::Domain("V3 vulnerability overflow"))
}

fn layer_flux(
    case: &V3PotentialCase,
    layer: &LayerOperands,
    root_potential_mm: f64,
    lai: f64,
    sai: f64,
) -> Result<f64, VegetationError> {
    if !layer.accessible || layer.frozen || layer.root_fraction == 0.0 {
        return Ok(0.0);
    }
    let p = &case.parameters;
    let soil_vulnerability = vulnerability(layer.soil_potential_mm, p.p50_root, p.ck)?;
    let kr = p.k3_max_m_s / layer.z3_m * soil_vulnerability;
    let ks = layer.ksoil_m2_s / layer.dxroot_m;
    let series = kr * ks / (kr + ks);
    let rai = (lai + sai) * layer.root_fraction * p.root_to_leaf_area;
    let flux = series * rai * (layer.soil_potential_mm - root_potential_mm + layer.gravity_head_mm);
    flux.is_finite()
        .then_some(flux)
        .ok_or(VegetationError::Domain("V3 layer hydraulic flux"))
}

fn validate_case(case: &V3PotentialCase) -> Result<(), VegetationError> {
    let f = &case.gas_energy;
    let p = &case.parameters;
    let b = &case.biochemical_parameters;
    let d = &case.surface_dimensions;
    let scalar_values = [
        case.tile_fraction,
        case.dt_s,
        f.pressure_pa,
        f.ca_pa,
        f.g0_umol_m2_s,
        f.medlyn_g1_kpa_sqrt,
        SPECIFIC_HEAT_AIR_J_KG_K,
        f.latent_heat_j_kg,
        f.rdry_j_kg_k,
        f.air_temperature_k,
        f.air_specific_humidity_kg_kg,
        f.rah_s_m,
        f.raw_s_m,
        f.stem_area,
        f.wet_fraction,
        f.canopy_liquid_kg_m2_tile,
        p.k1a_max_s1,
        p.k1b_max_s1,
        p.k2_max,
        p.height_m,
        p.k3_max_m_s,
        p.root_to_leaf_area,
        p.p50_root,
        p.p50_xylem,
        p.p50_leaf,
        p.ck,
        d.leaf_m,
        d.wet_surface_m,
        d.stem_m,
        case.classes.sun.leaf_area,
        case.classes.sun.absorbed_par_w_m2_leaf,
        case.classes.sun.absorbed_shortwave_w_m2_tile,
        case.classes.sun.vcmax25,
        case.classes.sun.jmax25,
        case.classes.sun.rd25,
        case.classes.sun.temperature_start_k,
        case.classes.shade.leaf_area,
        case.classes.shade.absorbed_par_w_m2_leaf,
        case.classes.shade.absorbed_shortwave_w_m2_tile,
        case.classes.shade.vcmax25,
        case.classes.shade.jmax25,
        case.classes.shade.rd25,
        case.classes.shade.temperature_start_k,
        b.kc25_pa,
        b.ko25_pa,
        b.gamma25_pa,
        b.ha_vcmax_j_mol,
        b.hd_vcmax_j_mol,
        b.entropy_vcmax_j_mol_k,
        b.ha_jmax_j_mol,
        b.hd_jmax_j_mol,
        b.entropy_jmax_j_mol_k,
        b.ha_kc_j_mol,
        b.ha_ko_j_mol,
        b.ha_gamma_j_mol,
        b.tp_vcmax_ratio,
        b.oxygen_mole_fraction,
        b.electron_quantum_yield,
        b.par_photon_umol_per_j,
        b.electron_curvature,
        b.ac_aj_curvature,
        b.ag_ap_curvature,
        f.gb_leaf_m_s,
        f.gb_wet_m_s,
        f.gb_stem_m_s,
        f.leaf_emissivity,
        f.wet_emissivity,
        f.stem_emissivity,
        f.longwave_down_w_m2,
        f.longwave_up_w_m2,
        f.stem_absorbed_shortwave_w_m2_tile,
        f.wet_temperature_start_k,
        f.stem_temperature_start_k,
        f.canopy_air_temperature_start_k,
        f.qcan_start_kg_kg,
    ];
    if scalar_values.iter().any(|value| !value.is_finite())
        || case.tile_fraction <= 0.0
        || case.tile_fraction > 1.0
        || case.dt_s <= 0.0
        || f.dt_s.to_bits() != case.dt_s.to_bits()
        || f.pressure_pa <= 0.0
        || f.ca_pa <= 0.0
        || f.g0_umol_m2_s < 0.0
        || f.medlyn_g1_kpa_sqrt < 0.0
        || f.cp_air_j_kg_k <= 0.0
        || f.latent_heat_j_kg <= 0.0
        || f.rdry_j_kg_k <= 0.0
        || f.air_specific_humidity_kg_kg < 0.0
        || f.rah_s_m <= 0.0
        || f.raw_s_m <= 0.0
        || !(0.0..=1.0).contains(&f.wet_fraction)
        || f.canopy_liquid_kg_m2_tile < 0.0
        || p.k1a_max_s1 <= 0.0
        || p.k1b_max_s1 <= 0.0
        || p.k2_max <= 0.0
        || p.height_m <= 0.0
        || p.k3_max_m_s <= 0.0
        || p.root_to_leaf_area <= 0.0
        || p.p50_root >= 0.0
        || p.p50_xylem >= 0.0
        || p.p50_leaf >= 0.0
        || p.ck <= 0.0
        || case.layers.is_empty()
        || d.leaf_m <= 0.0
        || d.wet_surface_m <= 0.0
        || d.stem_m <= 0.0
        || !valid_class(&case.classes.sun)
        || !valid_class(&case.classes.shade)
        || b.kc25_pa <= 0.0
        || b.ko25_pa <= 0.0
        || b.gamma25_pa < 0.0
        || b.ha_vcmax_j_mol <= 0.0
        || b.hd_vcmax_j_mol <= 0.0
        || b.entropy_vcmax_j_mol_k <= 0.0
        || b.ha_jmax_j_mol <= 0.0
        || b.hd_jmax_j_mol <= 0.0
        || b.entropy_jmax_j_mol_k <= 0.0
        || b.ha_kc_j_mol <= 0.0
        || b.ha_ko_j_mol <= 0.0
        || b.ha_gamma_j_mol <= 0.0
        || b.tp_vcmax_ratio <= 0.0
        || b.oxygen_mole_fraction.to_bits() != OXYGEN_MOLE_FRACTION.to_bits()
        || b.electron_quantum_yield.to_bits() != ELECTRON_QUANTUM_YIELD.to_bits()
        || b.par_photon_umol_per_j.to_bits() != PAR_PHOTON_UMOL_PER_J.to_bits()
        || b.electron_curvature.to_bits() != ELECTRON_CURVATURE.to_bits()
        || b.ac_aj_curvature.to_bits() != AC_AJ_CURVATURE.to_bits()
        || b.ag_ap_curvature.to_bits() != AG_AP_CURVATURE.to_bits()
        || f.cp_air_j_kg_k.to_bits() != SPECIFIC_HEAT_AIR_J_KG_K.to_bits()
        || f.rdry_j_kg_k.to_bits() != AIR_DENSITY_GAS_CONSTANT.to_bits()
        || f.leaf_emissivity <= 0.0
        || f.leaf_emissivity > 1.0
        || f.wet_emissivity <= 0.0
        || f.wet_emissivity > 1.0
        || f.stem_emissivity <= 0.0
        || f.stem_emissivity > 1.0
        || f.gb_leaf_m_s <= 0.0
        || f.gb_wet_m_s <= 0.0
        || f.gb_stem_m_s <= 0.0
        || !(273.15..=373.15).contains(&f.air_temperature_k)
        || !(273.15..=373.15).contains(&f.wet_temperature_start_k)
        || !(273.15..=373.15).contains(&f.stem_temperature_start_k)
        || !(273.15..=373.15).contains(&f.canopy_air_temperature_start_k)
        || f.qcan_start_kg_kg < 0.0
        || f.stem_area < 0.0
        || f.stem_absorbed_shortwave_w_m2_tile < 0.0
    {
        return Err(VegetationError::Domain("V3 potential constitutive case"));
    }
    let wind = &f.reference_wind_operands;
    if wind.kappa.to_bits() != 0.4_f64.to_bits() {
        return Err(VegetationError::Domain("V3 von Karman identity"));
    }
    let u_star = canopy_surface_friction_velocity(
        wind.u_ref_m_s,
        wind.z_ref_m,
        wind.displacement_m,
        wind.z0m_m,
    )?;
    require_derived_match(u_star, f.derived_u_star_m_s, "V3 u_star binding")?;
    require_derived_match(
        BOUNDARY_CONDUCTANCE_COEFFICIENT_M_S_HALF * (u_star / d.leaf_m).sqrt(),
        f.gb_leaf_m_s,
        "V3 leaf boundary conductance binding",
    )?;
    require_derived_match(
        BOUNDARY_CONDUCTANCE_COEFFICIENT_M_S_HALF * (u_star / d.wet_surface_m).sqrt(),
        f.gb_wet_m_s,
        "V3 wet boundary conductance binding",
    )?;
    require_derived_match(
        BOUNDARY_CONDUCTANCE_COEFFICIENT_M_S_HALF * (u_star / d.stem_m).sqrt(),
        f.gb_stem_m_s,
        "V3 stem boundary conductance binding",
    )?;
    require_derived_match(
        neutral_resistance(
            wind.z_ref_m,
            wind.displacement_m,
            wind.z0m_m,
            wind.z0h_m,
            wind.u_ref_m_s,
        )?,
        f.rah_s_m,
        "V3 heat resistance binding",
    )?;
    require_derived_match(
        neutral_resistance(
            wind.z_ref_m,
            wind.displacement_m,
            wind.z0m_m,
            wind.z0q_m,
            wind.u_ref_m_s,
        )?,
        f.raw_s_m,
        "V3 vapor resistance binding",
    )?;
    if f.latent_heat_j_kg.to_bits() != LATENT_HEAT_VAPORIZATION.to_bits() {
        return Err(VegetationError::Domain("V3 latent heat identity"));
    }
    let root_sum = case
        .layers
        .iter()
        .map(|layer| layer.root_fraction)
        .sum::<f64>();
    if (root_sum - 1.0).abs() > 1.0e-12 {
        return Err(VegetationError::Domain("V3 root-fraction closure"));
    }
    let mut layer_ids = BTreeSet::new();
    for layer in &case.layers {
        if [
            layer.soil_potential_mm,
            layer.gravity_head_mm,
            layer.root_fraction,
            layer.z3_m,
            layer.ksoil_m2_s,
            layer.dxroot_m,
        ]
        .iter()
        .any(|value| !value.is_finite())
            || layer.root_fraction < 0.0
            || layer.z3_m <= 0.0
            || layer.ksoil_m2_s < 0.0
            || layer.dxroot_m <= 0.0
            || !layer_ids.insert(&layer.layer_id)
            || SoilLayerId::try_new(layer.layer_id.clone()).is_err()
        {
            return Err(VegetationError::Domain("V3 hydraulic layer operands"));
        }
    }
    Ok(())
}

fn valid_class(class: &LeafClassOperands) -> bool {
    if class.leaf_area == 0.0 {
        class.absorbed_par_w_m2_leaf == 0.0
            && class.absorbed_shortwave_w_m2_tile == 0.0
            && class.vcmax25 == 0.0
            && class.jmax25 == 0.0
            && class.rd25 == 0.0
            && (273.15..=373.15).contains(&class.temperature_start_k)
    } else {
        class.leaf_area > 0.0
            && class.absorbed_par_w_m2_leaf >= 0.0
            && class.absorbed_shortwave_w_m2_tile >= 0.0
            && class.vcmax25 > 0.0
            && class.jmax25 > 0.0
            && class.rd25 > 0.0
            && (273.15..=373.15).contains(&class.temperature_start_k)
    }
}

fn require_derived_match(
    computed: f64,
    bound: f64,
    identity: &'static str,
) -> Result<(), VegetationError> {
    let tolerance = 2.0e-12 * computed.abs().max(bound.abs()).max(1.0e-12);
    if computed.is_finite() && bound.is_finite() && (computed - bound).abs() <= tolerance {
        Ok(())
    } else {
        Err(VegetationError::Domain(identity))
    }
}

#[cfg(test)]
mod tests {
    use openwepp_kernel_contract::{OccupancyId, StratumId, TileId, TransactionId};
    use serde_json::Value;

    use super::*;
    use crate::occupancy_solver::potential::{StageASolveIdentity, solve_uncapped_stage_a};

    const VECTORS: &str = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../docs/work-packages/20260812-c3-woody-potential-pass-authority-001/artifacts/openwepp_c3_woody_v3_vectors.json"
    ));

    fn fixture() -> (V3PotentialCase, Value) {
        let root: Value = serde_json::from_str(VECTORS).expect("released V3 fixture parses");
        let family = root["families"]["hydraulic_potential_pass"].clone();
        let mut runtime = family["operands"].clone();
        runtime
            .as_object_mut()
            .expect("potential operands object")
            .remove("emax");
        runtime
            .as_object_mut()
            .expect("potential operands object")
            .insert(
                "surface_dimensions".into(),
                serde_json::json!({
                    "leaf_m": root["families"]["aerodynamics"]["operands"]["leaf_dimension_m"],
                    "wet_surface_m": root["families"]["aerodynamics"]["operands"]["wet_surface_dimension_m"],
                    "stem_m": root["families"]["aerodynamics"]["operands"]["stem_dimension_m"]
                }),
            );
        let parameters = runtime["parameters"]
            .as_object_mut()
            .expect("hydraulic parameters object");
        let k1 = parameters.remove("k1_max").expect("fixture k1 maximum");
        parameters.remove("stem_to_leaf_path_m");
        parameters.remove("sun_leaf_area");
        parameters.remove("shade_leaf_area");
        parameters.remove("sai");
        parameters.remove("lai");
        parameters.insert("k1a_max_s1".into(), k1.clone());
        parameters.insert("k1b_max_s1".into(), k1);
        let biochemistry = runtime["biochemical_parameters"]
            .as_object_mut()
            .expect("biochemistry object");
        biochemistry.remove("oxygen_partial_pressure_pa");
        biochemistry.insert("oxygen_mole_fraction".into(), serde_json::json!(0.20));
        let case =
            serde_json::from_value(runtime).expect("exact V3 potential operands deserialize");
        (case, family)
    }

    fn identity() -> StageASolveIdentity {
        StageASolveIdentity {
            transaction_id: TransactionId(17),
            occupancy_id: OccupancyId {
                stratum_id: StratumId::try_new("upper").expect("stratum"),
                tile_id: TileId::try_new("tile-a").expect("tile"),
            },
        }
    }

    fn context() -> ConstitutiveSolveContext {
        let identity = identity();
        ConstitutiveSolveContext {
            transaction_id: identity.transaction_id,
            occupancy_id: identity.occupancy_id,
            pass: CoupledSolvePass::Potential,
        }
    }

    fn state(values: &[f64]) -> StageAState {
        StageAState {
            psi_sunleaf_mm: values[0],
            psi_shadeleaf_mm: values[1],
            psi_stem_mm: values[2],
            psi_root_mm: values[3],
            beta_sun: values[4],
            beta_shade: values[5],
        }
    }

    fn close(actual: f64, expected: f64, atol: f64, rtol: f64) {
        assert!(
            (actual - expected).abs() <= atol + rtol * expected.abs(),
            "actual={actual:.17e}, expected={expected:.17e}"
        );
    }

    #[test]
    fn beta_one_maximum_is_nested_and_matches_independent_fixture() {
        let (case, family) = fixture();
        let maximum = solve_canopy_energy(&case, (1.0, 1.0), (-5_900.0, -5_450.0), &context())
            .expect("complete beta-one maximum solve");
        let evaluator = V3ConstitutiveEvaluator::new(case, (-5_900.0, -5_450.0), context())
            .expect("internal maximum evaluation");
        close(
            evaluator.maximum_demand().sun,
            family["internal_maximum_evaluation"]["emax"]["sun"]
                .as_f64()
                .expect("sun Emax"),
            1.0e-12,
            0.0,
        );
        let expected = &family["internal_maximum_evaluation"];
        let canopy = &expected["canopy_energy_state"];
        close(
            maximum.canopy_air_temperature_k,
            canopy["canopy_air_temperature_k"]
                .as_f64()
                .expect("maximum canopy temperature"),
            ENERGY_STEP_TOLERANCE_K,
            0.0,
        );
        close(
            maximum.canopy_air_specific_humidity_kg_kg,
            canopy["canopy_air_specific_humidity_kg_kg"]
                .as_f64()
                .expect("maximum canopy humidity"),
            1.0e-12,
            0.0,
        );
        close(
            maximum.wet_surface_temperature_k,
            canopy["wet_surface_temperature_k"]
                .as_f64()
                .expect("maximum wet temperature"),
            ENERGY_STEP_TOLERANCE_K,
            0.0,
        );
        close(
            maximum.sun.ci_pa,
            expected["sun_gas_energy_state"]["ci_pa"]
                .as_f64()
                .expect("maximum sun ci"),
            1.0e-6,
            0.0,
        );
        close(
            maximum.shade.ci_pa,
            expected["shade_gas_energy_state"]["ci_pa"]
                .as_f64()
                .expect("maximum shade ci"),
            1.0e-6,
            0.0,
        );
        assert!(maximum.wet_store_cap_active);
        assert_eq!(
            maximum.iterations,
            u32::try_from(canopy["iterations"].as_u64().expect("iterations"))
                .expect("iteration count fits u32")
        );
        assert_eq!(maximum.backtracking_count, 0);
        assert_eq!(maximum.sun.ci_iterations, 6);
        assert_eq!(maximum.shade.ci_iterations, 6);
        close(
            evaluator.maximum_demand().shade,
            family["internal_maximum_evaluation"]["emax"]["shade"]
                .as_f64()
                .expect("shade Emax"),
            1.0e-12,
            0.0,
        );
    }

    #[test]
    fn uncapped_stage_a_matches_independent_fixture_from_both_warm_starts() {
        let (case, family) = fixture();
        let evaluator = V3ConstitutiveEvaluator::new(case, (-5_900.0, -5_450.0), context())
            .expect("constitutive evaluator");
        for (start_path, result_path) in [
            ("accepted_uncapped_stage_a", "accepted_uncapped_stage_a"),
            ("alternate_warm_start", "alternate_warm_start"),
        ] {
            let start = if start_path == "accepted_uncapped_stage_a" {
                vec![-5_900.0, -5_450.0, -4_300.0, -2_850.0, 0.68, 0.66]
            } else {
                family[start_path]["start"]
                    .as_array()
                    .expect("alternate start")
                    .iter()
                    .map(|value| value.as_f64().expect("number"))
                    .collect()
            };
            let solved = solve_uncapped_stage_a(&identity(), state(&start), &evaluator)
                .expect("complete V3 coupled solve");
            let expected = if result_path == "accepted_uncapped_stage_a" {
                &family[result_path]["solution"]
            } else {
                &family[result_path]["result"]["solution"]
            };
            let expected_diagnostics = if result_path == "accepted_uncapped_stage_a" {
                &family[result_path]
            } else {
                &family[result_path]["result"]
            };
            close(
                solved.state.psi_sunleaf_mm,
                expected["sun_leaf_potential_mm"].as_f64().expect("sun psi"),
                1.0e-7,
                0.0,
            );
            close(
                solved.state.psi_shadeleaf_mm,
                expected["shade_leaf_potential_mm"]
                    .as_f64()
                    .expect("shade psi"),
                1.0e-7,
                0.0,
            );
            close(
                solved.state.psi_stem_mm,
                expected["stem_potential_mm"].as_f64().expect("stem psi"),
                1.0e-7,
                0.0,
            );
            close(
                solved.state.psi_root_mm,
                expected["root_node_potential_mm"]
                    .as_f64()
                    .expect("root psi"),
                1.0e-7,
                0.0,
            );
            close(
                solved.state.beta_sun,
                expected["beta_hyd_sun"].as_f64().expect("sun beta"),
                1.0e-9,
                0.0,
            );
            close(
                solved.state.beta_shade,
                expected["beta_hyd_shade"].as_f64().expect("shade beta"),
                1.0e-9,
                0.0,
            );
            close(
                solved.persisted_beta_hyd,
                expected["beta_hyd"].as_f64().expect("aggregate beta"),
                1.0e-9,
                0.0,
            );
            assert!(
                solved
                    .normalized_residuals
                    .iter()
                    .all(|residual| residual.value.abs() <= 1.0)
            );
            assert_eq!(
                solved.iterations,
                u32::try_from(
                    expected_diagnostics["iterations"]
                        .as_u64()
                        .expect("outer iterations"),
                )
                .expect("iteration count fits u32")
            );
            assert_eq!(
                solved.backtracking_count,
                u32::try_from(
                    expected_diagnostics["backtracking_count"]
                        .as_u64()
                        .expect("outer backtracking"),
                )
                .expect("backtracking count fits u32")
            );
            assert!(solved.potential_step_mm <= 1.0e-7);
            assert_eq!(
                solved.normalized_residuals.len(),
                expected_diagnostics["normalized_residuals"]
                    .as_array()
                    .expect("outer residuals")
                    .len()
            );
        }
    }

    #[test]
    fn accepted_fluxes_and_layer_requests_match_independent_fixture() {
        let (case, family) = fixture();
        let tile_fraction = case.tile_fraction;
        let dt_s = case.dt_s;
        let expected_solution = &family["accepted_uncapped_stage_a"]["solution"];
        let expected_flux = &family["accepted_uncapped_stage_a"]["fluxes"];
        let accepted_energy = solve_canopy_energy(
            &case,
            (
                expected_solution["beta_hyd_sun"]
                    .as_f64()
                    .expect("accepted sun beta"),
                expected_solution["beta_hyd_shade"]
                    .as_f64()
                    .expect("accepted shade beta"),
            ),
            (
                expected_solution["sun_leaf_potential_mm"]
                    .as_f64()
                    .expect("accepted sun potential"),
                expected_solution["shade_leaf_potential_mm"]
                    .as_f64()
                    .expect("accepted shade potential"),
            ),
            &context(),
        )
        .expect("accepted nested gas-energy solve");
        assert_eq!(
            accepted_energy.iterations,
            u32::try_from(
                expected_flux["canopy_energy_state"]["iterations"]
                    .as_u64()
                    .expect("accepted energy iterations"),
            )
            .expect("iteration count fits u32")
        );
        let evaluator = V3ConstitutiveEvaluator::new(case, (-5_900.0, -5_450.0), context())
            .expect("constitutive evaluator");
        let solved = solve_uncapped_stage_a(
            &identity(),
            state(&[-5_900.0, -5_450.0, -4_300.0, -2_850.0, 0.68, 0.66]),
            &evaluator,
        )
        .expect("coupled solution");
        close(
            solved.evaluation.q1_sun_kg_m2_s,
            expected_flux["q1_sun"].as_f64().expect("q1 sun"),
            2.0e-13,
            2.0e-9,
        );
        close(
            solved.evaluation.q1_shade_kg_m2_s,
            expected_flux["q1_shade"].as_f64().expect("q1 shade"),
            2.0e-13,
            2.0e-9,
        );
        close(
            solved.evaluation.q2_kg_m2_s,
            expected_flux["q2"].as_f64().expect("q2"),
            2.0e-13,
            2.0e-9,
        );
        let expected_canopy = &expected_flux["canopy_energy_state"];
        close(
            accepted_energy.canopy_air_temperature_k,
            expected_canopy["canopy_air_temperature_k"]
                .as_f64()
                .expect("accepted canopy temperature"),
            2.0e-8,
            2.0e-10,
        );
        close(
            accepted_energy.dry_stem_temperature_k,
            expected_canopy["dry_stem_temperature_k"]
                .as_f64()
                .expect("accepted stem temperature"),
            2.0e-8,
            2.0e-10,
        );
        close(
            accepted_energy.wet_actual_kg_m2_s,
            expected_canopy["wet_actual_kg_m2_s"]
                .as_f64()
                .expect("accepted wet vapor"),
            2.0e-13,
            2.0e-9,
        );
        close(
            accepted_energy.sun.transpiration_kg_m2_tile_s,
            expected_flux["gas_energy_transpiration_sun"]
                .as_f64()
                .expect("accepted sun transpiration"),
            2.0e-13,
            2.0e-9,
        );
        close(
            accepted_energy.shade.transpiration_kg_m2_tile_s,
            expected_flux["gas_energy_transpiration_shade"]
                .as_f64()
                .expect("accepted shade transpiration"),
            2.0e-13,
            2.0e-9,
        );
        let expected_requests = family["accepted_uncapped_stage_a"]["water_requests"]
            .as_array()
            .expect("requests");
        assert_eq!(solved.evaluation.q3_kg_m2_s.len(), expected_requests.len());
        for ((layer_id, flux), expected) in
            solved.evaluation.q3_kg_m2_s.iter().zip(expected_requests)
        {
            assert_eq!(
                layer_id.as_str(),
                expected["layer_id"].as_str().expect("layer")
            );
            close(
                tile_fraction * flux * dt_s,
                expected["amount_kg_h2o_m2_stand_ground"]
                    .as_f64()
                    .expect("amount"),
                2.0e-13,
                2.0e-9,
            );
        }
    }

    #[test]
    fn accepted_nested_solution_populates_all_fifteen_occupancy_lanes() {
        let (case, _) = fixture();
        let evaluator = V3ConstitutiveEvaluator::new(case, (-5_900.0, -5_450.0), context())
            .expect("constitutive evaluator");
        let accepted = evaluator
            .solve_uncapped(
                &identity(),
                state(&[-5_900.0, -5_450.0, -4_300.0, -2_850.0, 0.68, 0.66]),
            )
            .expect("accepted nested Stage-A");
        let beginning = OccupancyState {
            beta_hyd: 0.67,
            canopy_air_specific_humidity_kg_kg: 0.011,
            canopy_air_temperature_k: 295.8,
            canopy_liquid_kg_h2o_m2_tile_ground: 0.018,
            dry_stem_temperature_k: 295.2,
            last_accepted_transaction_id: Some(16),
            root_node_potential_mm: -2_850.0,
            shade_ci_pa: 30.0,
            shade_leaf_potential_mm: -5_450.0,
            shade_leaf_temperature_k: 295.4,
            stem_potential_mm: -4_300.0,
            sun_ci_pa: 30.0,
            sun_leaf_potential_mm: -5_900.0,
            sun_leaf_temperature_k: 296.2,
            wet_surface_temperature_k: 295.6,
        };
        let candidate = accepted
            .occupancy_state(&beginning, 0.012)
            .expect("all accepted occupancy lanes");
        assert_eq!(candidate.last_accepted_transaction_id, Some(16));
        close(
            candidate.beta_hyd,
            accepted.outer.persisted_beta_hyd,
            0.0,
            0.0,
        );
        close(candidate.sun_ci_pa, accepted.canopy.sun.ci_pa, 0.0, 0.0);
        close(candidate.shade_ci_pa, accepted.canopy.shade.ci_pa, 0.0, 0.0);
        assert_eq!(
            serde_json::to_value(candidate)
                .expect("candidate serializes")
                .as_object()
                .expect("state object")
                .len(),
            15
        );
        assert!(!accepted.canopy.normalized_residuals.is_empty());
        assert!(accepted.canopy.sun.ci_iterations > 0);
        assert!(accepted.canopy.shade.ci_iterations > 0);
        assert!(accepted.canopy.pivot_magnitude.is_some());
        assert!(accepted.canopy.matrix_norm.is_some());
    }

    #[test]
    fn wrong_derived_wind_and_redistribution_fail_closed() {
        let (mut case, family) = fixture();
        let runtime_template = case.clone();
        case.gas_energy.derived_u_star_m_s = case.gas_energy.reference_wind_operands.u_ref_m_s;
        assert!(V3ConstitutiveEvaluator::new(case, (-5_900.0, -5_450.0), context()).is_err());

        let mut poison = runtime_template;
        poison.layers[1].soil_potential_mm = family["redistribution_poison"]["operands"]["layers"]
            [1]["soil_potential_mm"]
            .as_f64()
            .expect("poison soil potential");
        let evaluator = V3ConstitutiveEvaluator::new(poison, (-5_900.0, -5_450.0), context())
            .expect("poison evaluator construction");
        let initial = state(&[-5_900.0, -5_450.0, -4_300.0, -2_850.0, 0.68, 0.66]);
        let trial = evaluator
            .evaluate(initial)
            .expect("negative layer flux remains visible to coupled Newton trials");
        assert!(trial.q3_kg_m2_s.iter().any(|(_, flux)| *flux < 0.0));
        assert!(solve_uncapped_stage_a(&identity(), initial, &evaluator).is_err());
    }

    fn deactivate(class: &mut LeafClassOperands) {
        class.leaf_area = 0.0;
        class.absorbed_par_w_m2_leaf = 0.0;
        class.absorbed_shortwave_w_m2_tile = 0.0;
        class.vcmax25 = 0.0;
        class.jmax25 = 0.0;
        class.rd25 = 0.0;
    }

    #[test]
    fn zero_direct_and_zero_lai_classes_are_exact_zero_branches() {
        let (mut zero_direct, _) = fixture();
        deactivate(&mut zero_direct.classes.sun);
        let evaluator = V3ConstitutiveEvaluator::new(zero_direct, (-5_900.0, -5_450.0), context())
            .expect("zero-direct inactive sun class");
        assert_eq!(evaluator.maximum_demand().sun.to_bits(), 0.0_f64.to_bits());
        let trial = evaluator
            .evaluate(state(&[-5_900.0, -5_450.0, -4_300.0, -2_850.0, 1.0, 0.66]))
            .expect("zero-direct constitutive evaluation");
        assert_eq!(trial.gas_sun_kg_m2_s.to_bits(), 0.0_f64.to_bits());
        assert_eq!(trial.q1_sun_kg_m2_s.to_bits(), 0.0_f64.to_bits());
        assert_eq!(
            trial.vulnerability_demand_sun_kg_m2_s.to_bits(),
            0.0_f64.to_bits()
        );

        let (mut zero_lai, _) = fixture();
        deactivate(&mut zero_lai.classes.sun);
        deactivate(&mut zero_lai.classes.shade);
        let evaluator = V3ConstitutiveEvaluator::new(zero_lai, (-5_900.0, -5_450.0), context())
            .expect("zero-LAI evaluator");
        let solved = solve_uncapped_stage_a(
            &identity(),
            state(&[-5_900.0, -5_450.0, -4_300.0, -2_850.0, 0.2, 0.3]),
            &evaluator,
        )
        .expect("zero-LAI exact coupled branch");
        assert_eq!(solved.state.beta_sun.to_bits(), 1.0_f64.to_bits());
        assert_eq!(solved.state.beta_shade.to_bits(), 1.0_f64.to_bits());
        assert_eq!(solved.persisted_beta_hyd.to_bits(), 1.0_f64.to_bits());
        assert!(
            solved
                .evaluation
                .q3_kg_m2_s
                .iter()
                .all(|(_, flux)| flux.to_bits() == 0.0_f64.to_bits())
        );
        let accepted = evaluator
            .solve_uncapped(
                &identity(),
                state(&[-5_900.0, -5_450.0, -4_300.0, -2_850.0, 0.2, 0.3]),
            )
            .expect("zero-LAI accepted nested state");
        assert_eq!(
            accepted.canopy.sun.transpiration_kg_m2_tile_s.to_bits(),
            0.0_f64.to_bits()
        );
        assert_eq!(
            accepted.canopy.shade.transpiration_kg_m2_tile_s.to_bits(),
            0.0_f64.to_bits()
        );
    }

    #[test]
    fn zero_area_energy_nodes_use_explicit_nonsingular_branches() {
        let (mut dry, _) = fixture();
        dry.gas_energy.wet_fraction = 0.0;
        dry.gas_energy.canopy_liquid_kg_m2_tile = 0.0;
        let dry_result = solve_canopy_energy(&dry, (1.0, 1.0), (-5_900.0, -5_450.0), &context())
            .expect("dry canopy has an explicit inactive wet node");
        assert_eq!(dry_result.wet_actual_kg_m2_s.to_bits(), 0.0_f64.to_bits());
        close(
            dry_result.wet_surface_temperature_k,
            dry_result.canopy_air_temperature_k,
            ENERGY_STEP_TOLERANCE_K,
            0.0,
        );

        let (mut fully_wet, _) = fixture();
        fully_wet.gas_energy.wet_fraction = 1.0;
        let fully_wet_result =
            solve_canopy_energy(&fully_wet, (1.0, 1.0), (-5_900.0, -5_450.0), &context())
                .expect("fully wet canopy has explicit inactive dry nodes");
        close(
            fully_wet_result.dry_stem_temperature_k,
            fully_wet_result.canopy_air_temperature_k,
            ENERGY_STEP_TOLERANCE_K,
            0.0,
        );
        assert_eq!(
            fully_wet_result.sun.transpiration_kg_m2_tile_s.to_bits(),
            0.0_f64.to_bits()
        );
        assert_eq!(
            fully_wet_result.shade.transpiration_kg_m2_tile_s.to_bits(),
            0.0_f64.to_bits()
        );

        let (mut no_stem, _) = fixture();
        no_stem.gas_energy.stem_area = 0.0;
        no_stem.gas_energy.stem_absorbed_shortwave_w_m2_tile = 0.0;
        let no_stem_result =
            solve_canopy_energy(&no_stem, (1.0, 1.0), (-5_900.0, -5_450.0), &context())
                .expect("zero stem area has an explicit inactive dry-stem node");
        close(
            no_stem_result.dry_stem_temperature_k,
            no_stem_result.canopy_air_temperature_k,
            ENERGY_STEP_TOLERANCE_K,
            0.0,
        );

        let (mut empty, _) = fixture();
        deactivate(&mut empty.classes.sun);
        deactivate(&mut empty.classes.shade);
        empty.gas_energy.stem_area = 0.0;
        empty.gas_energy.stem_absorbed_shortwave_w_m2_tile = 0.0;
        empty.gas_energy.wet_fraction = 0.0;
        empty.gas_energy.canopy_liquid_kg_m2_tile = 0.0;
        let empty_result =
            solve_canopy_energy(&empty, (1.0, 1.0), (-5_900.0, -5_450.0), &context())
                .expect("empty plant area has an exact nonsingular energy branch");
        assert_eq!(empty_result.wet_actual_kg_m2_s.to_bits(), 0.0_f64.to_bits());
        assert_eq!(
            empty_result.sun.transpiration_kg_m2_tile_s.to_bits(),
            0.0_f64.to_bits()
        );
        assert_eq!(
            empty_result.shade.transpiration_kg_m2_tile_s.to_bits(),
            0.0_f64.to_bits()
        );
    }

    #[test]
    fn runtime_schema_excludes_expected_emax_and_uses_configured_dimensions() {
        let (case, family) = fixture();
        let runtime = serde_json::to_value(&case).expect("runtime DTO serializes canonically");
        assert!(runtime.get("emax").is_none());
        assert!(
            runtime["biochemical_parameters"]
                .get("oxygen_partial_pressure_pa")
                .is_none()
        );

        let mut wrong_dimension = case.clone();
        wrong_dimension.surface_dimensions.leaf_m *= 2.0;
        assert!(
            V3ConstitutiveEvaluator::new(wrong_dimension, (-5_900.0, -5_450.0), context(),)
                .is_err()
        );
        assert!(family["operands"]["emax"].is_object());

        let mut nonstandard_pressure = case.clone();
        nonstandard_pressure.gas_energy.pressure_pa = 90_000.0;
        V3ConstitutiveEvaluator::new(nonstandard_pressure, (-5_900.0, -5_450.0), context())
            .expect("oxygen partial pressure derives from valid nonstandard pressure");

        let mut fixed_partial_poison = runtime;
        fixed_partial_poison["biochemical_parameters"]
            .as_object_mut()
            .expect("biochemistry")
            .insert(
                "oxygen_partial_pressure_pa".into(),
                serde_json::json!(18_000.0),
            );
        assert!(serde_json::from_value::<V3PotentialCase>(fixed_partial_poison).is_err());

        let serialized = serde_json::to_value(&case).expect("runtime DTO serializes");
        for duplicate in ["sun_leaf_area", "shade_leaf_area", "lai", "sai"] {
            let mut poison = serialized.clone();
            poison["parameters"]
                .as_object_mut()
                .expect("hydraulic parameters")
                .insert(duplicate.into(), serde_json::json!(1.0));
            assert!(serde_json::from_value::<V3PotentialCase>(poison).is_err());
        }
    }

    #[test]
    fn nested_ci_and_energy_failures_retain_canonical_context() {
        let (mut case, family) = fixture();
        case.classes.sun.absorbed_par_w_m2_leaf = f64::NAN;
        assert!(matches!(
            V3ConstitutiveEvaluator::new(case, (-5_900.0, -5_450.0), context()),
            Err(VegetationError::Domain("V3 potential constitutive case"))
        ));

        let (case, _) = fixture();
        let mut energy_domain = case.clone();
        energy_domain.gas_energy.canopy_air_temperature_start_k = 250.0;
        assert!(matches!(
            V3ConstitutiveEvaluator::new(energy_domain, (-5_900.0, -5_450.0), context(),),
            Err(VegetationError::Domain("V3 potential constitutive case"))
        ));

        let (mut ci_domain, _) = fixture();
        ci_domain.gas_energy.qcan_start_kg_kg = 0.1;
        let failure = V3ConstitutiveEvaluator::new(ci_domain, (-5_900.0, -5_450.0), context())
            .expect_err("surface-VPD domain must retain class identity");
        let VegetationError::NumericalFailure {
            category,
            diagnostics: ci_domain_diagnostics,
        } = failure
        else {
            panic!("typed class failure required");
        };
        assert_eq!(category, NumericalFailureCategory::Domain);
        assert_eq!(ci_domain_diagnostics.solve, SolveIdentity::SunCi);
        assert_eq!(ci_domain_diagnostics.iterations, 0);
        assert_eq!(ci_domain_diagnostics.bracket, None);

        let failure = brent_dekker_class(
            |value| {
                Ok((
                    value.mul_add(value, 1.0),
                    SolvedClass {
                        ci_pa: value,
                        rs_s_m: 1.0,
                        iterations: 0,
                        bracket: (-1.0, 1.0),
                    },
                ))
            },
            -1.0,
            1.0,
            64,
            &context(),
            SolveIdentity::SunCi,
        )
        .expect_err("unbracketed ci must fail");
        let VegetationError::NumericalFailure {
            category,
            diagnostics,
        } = failure
        else {
            panic!("typed numerical failure required");
        };
        assert_eq!(category, NumericalFailureCategory::BracketFailure);
        assert_eq!(diagnostics.solve, SolveIdentity::SunCi);
        assert_eq!(diagnostics.transaction_id, TransactionId(17));
        assert_eq!(diagnostics.occupancy_id, identity().occupancy_id);
        assert_eq!(diagnostics.bracket, Some((-1.0, 1.0)));
        assert_eq!(diagnostics.residual_norms.len(), 2);
        diagnostics.validate().expect("finite canonical payload");

        let (case, _) = fixture();
        let energy_failure =
            solve_canopy_energy_with_limit(&case, (0.6, 0.6), (-5_900.0, -5_450.0), &context(), 0)
                .expect_err("zero-iteration energy fixture must fail");
        let VegetationError::NumericalFailure {
            category,
            diagnostics: energy_diagnostics,
        } = energy_failure
        else {
            panic!("typed energy failure required");
        };
        assert_eq!(category, NumericalFailureCategory::IterationLimit);
        let expected = &family["executed_canopy_energy_failures"][1]["diagnostics"];
        assert_eq!(energy_diagnostics.solve, SolveIdentity::CanopyEnergy);
        assert_eq!(energy_diagnostics.iterations, 0);
        assert_eq!(energy_diagnostics.residual_norms.len(), 6);
        for (actual, expected) in energy_diagnostics.residual_norms.iter().zip(
            expected["residual_norms"]
                .as_array()
                .expect("energy residuals"),
        ) {
            assert_eq!(
                actual.identity,
                expected["identity"].as_str().expect("identity")
            );
            close(
                actual.value,
                expected["normalized"].as_f64().expect("normalized"),
                2.0e-5,
                2.0e-10,
            );
        }
        energy_diagnostics
            .validate()
            .expect("finite canonical energy payload");

        let singular = solve_linear([[0.0; 6]; 6], [0.0; 6])
            .expect_err("zero-norm Jacobian is explicitly singular");
        assert_eq!(singular.pivot_magnitude.to_bits(), 0.0_f64.to_bits());
        assert_eq!(singular.matrix_norm.to_bits(), 0.0_f64.to_bits());
    }
}
