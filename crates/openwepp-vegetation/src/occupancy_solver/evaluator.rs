//! Production V4 owner-uncapped occupancy evaluator.
//!
//! This adapter contains no constitutive alternatives. It binds validated
//! configuration, shared C/N state, occupancy-local warm starts, whole-column
//! radiation, snow-free forcing, and preliminary E04 liquid state to the
//! digest-bound V3 constitutive equations under the V4 shared-state ownership
//! amendment.

use std::collections::BTreeMap;

use openwepp_kernel_contract::SoilLayerId;

use super::capped_pass::CappedOccupancyEvaluator;
use super::constitutive::{
    BiochemicalParameters, ConstitutiveSolveContext, GasEnergyOperands, HydraulicParameters,
    LayerOperands, LeafClassOperands, LeafClasses, ReferenceWindOperands, SurfaceDimensions,
    V3ConstitutiveEvaluator, V3PotentialCase,
};
use super::potential::{StageASolveIdentity, StageAState};
use super::radiation::OccupancyRadiation;
use super::request_pass::PotentialOccupancyEvaluator;
use crate::VegetationError;
use crate::carbon_nitrogen::{Tissue, atkin_rd25, update_t10};
use crate::column::{OccupancyDiagnostics, OccupancyPassInput, OccupancyPassResult};
use crate::config::VegetationConfiguration;
use crate::diagnostics::CoupledSolvePass;
use crate::energy::{
    LATENT_HEAT_VAPORIZATION, VON_KARMAN, canopy_surface_friction_velocity,
    leaf_boundary_conductance, neutral_resistance,
};
use crate::interception::{InterceptionInput, InterceptionResult, liquid_interception};
use crate::radiation::OwnedLayerAbsorption;
use crate::transaction::SoilLayerForcing;

const CP_AIR_J_KG_K: f64 = 1_004.64;
const R_DRY_AIR_J_KG_K: f64 = 287.05;
const OXYGEN_MOLE_FRACTION: f64 = 0.20;
const ELECTRON_QUANTUM_YIELD: f64 = 0.85;
const PAR_PHOTON_UMOL_PER_J: f64 = 4.6;
const ELECTRON_CURVATURE: f64 = 0.7;
const AC_AJ_CURVATURE: f64 = 0.98;
const AG_AP_CURVATURE: f64 = 0.95;

/// Exact V4 production adapter for one occupancy-local owner-uncapped solve.
/// The evaluator is deliberately stateless: the exact interval and every
/// numerical warm start arrive through validated owner state/context. Persisted
/// `ci` lanes are validated and replaced by accepted endpoints, but they do
/// not alter the canonical Brent bracket `[Gamma*, ca]`.
#[derive(Clone, Copy, Debug, Default)]
pub(crate) struct ProductionPotentialOccupancyEvaluator;

/// Exact V4 production adapter for the owner-authorization-capped second pass.
#[derive(Clone, Copy, Debug, Default)]
pub(crate) struct ProductionCappedOccupancyEvaluator;

impl ProductionPotentialOccupancyEvaluator {
    pub(crate) fn from_configuration(
        configuration: &VegetationConfiguration,
    ) -> Result<Self, VegetationError> {
        configuration.validate()?;
        Ok(Self)
    }
}

impl PotentialOccupancyEvaluator for ProductionPotentialOccupancyEvaluator {
    fn solve_potential(
        &self,
        input: OccupancyPassInput<'_>,
        radiation: &OccupancyRadiation,
    ) -> Result<OccupancyPassResult, VegetationError> {
        solve_occupancy(&input, radiation, CoupledSolvePass::Potential)
    }
}

impl ProductionCappedOccupancyEvaluator {
    pub(crate) fn from_configuration(
        configuration: &VegetationConfiguration,
    ) -> Result<Self, VegetationError> {
        configuration.validate()?;
        Ok(Self)
    }
}

impl CappedOccupancyEvaluator for ProductionCappedOccupancyEvaluator {
    fn solve_capped(
        &self,
        input: OccupancyPassInput<'_>,
        radiation: &OccupancyRadiation,
    ) -> Result<OccupancyPassResult, VegetationError> {
        solve_occupancy(&input, radiation, CoupledSolvePass::Capped)
    }
}

#[allow(clippy::too_many_lines)]
fn solve_occupancy(
    input: &OccupancyPassInput<'_>,
    radiation: &OccupancyRadiation,
    pass: CoupledSolvePass,
) -> Result<OccupancyPassResult, VegetationError> {
    validate_identity(input, radiation, pass)?;

    let preliminary = interception(input, 0.0, input.occupancy_state.wet_surface_temperature_k)?;
    let advanced_t10_k = update_t10(
        input.shared_state.t10_k,
        input.forcing.air_temperature_k,
        input.interval_s,
    )?;
    let case = prepare_case(input, radiation, preliminary, advanced_t10_k)?;
    let context = ConstitutiveSolveContext {
        transaction_id: input.transaction_id,
        occupancy_id: input.occupancy_id.clone(),
        pass,
    };
    let maximum_leaf_potentials = (
        input.occupancy_state.sun_leaf_potential_mm,
        input.occupancy_state.shade_leaf_potential_mm,
    );
    let evaluator = match pass {
        CoupledSolvePass::Potential => {
            V3ConstitutiveEvaluator::new(case, maximum_leaf_potentials, context)?
        }
        CoupledSolvePass::Capped => V3ConstitutiveEvaluator::new_capped(
            case,
            maximum_leaf_potentials,
            context,
            local_cap_rates(input)?,
        )?,
    };
    let identity = StageASolveIdentity {
        transaction_id: input.transaction_id,
        occupancy_id: input.occupancy_id.clone(),
    };
    let initial = StageAState {
        psi_sunleaf_mm: input.occupancy_state.sun_leaf_potential_mm,
        psi_shadeleaf_mm: input.occupancy_state.shade_leaf_potential_mm,
        psi_stem_mm: input.occupancy_state.stem_potential_mm,
        psi_root_mm: input.occupancy_state.root_node_potential_mm,
        beta_sun: input.occupancy_state.beta_hyd,
        beta_shade: input.occupancy_state.beta_hyd,
    };
    let accepted = match pass {
        CoupledSolvePass::Potential => evaluator.solve_uncapped(&identity, initial)?,
        CoupledSolvePass::Capped => evaluator.solve_capped(&identity, initial)?,
    };

    let final_liquid = interception(
        input,
        accepted.canopy.wet_actual_kg_m2_s * input.interval_s,
        accepted.canopy.wet_surface_temperature_k,
    )?;
    let candidate_state = accepted.occupancy_state(input.occupancy_state, final_liquid.store1)?;
    let local_layer_water_kg_m2_tile_ground = accepted
        .outer
        .evaluation
        .q3_kg_m2_s
        .iter()
        .map(|(layer_id, flux)| (layer_id.clone(), flux * input.interval_s))
        .collect::<Vec<_>>();
    validate_layer_output(input, &local_layer_water_kg_m2_tile_ground)?;

    let mut normalized_residuals = accepted.outer.normalized_residuals.clone();
    normalized_residuals.extend(accepted.canopy.normalized_residuals.clone());
    let q3_sum = accepted
        .outer
        .evaluation
        .q3_kg_m2_s
        .iter()
        .map(|(_, flux)| *flux)
        .sum::<f64>();
    let gas_sum = accepted.canopy.sun.transpiration_kg_m2_tile_s
        + accepted.canopy.shade.transpiration_kg_m2_tile_s;
    let gas_hydraulic_mismatch_kg_m2_s = gas_sum - q3_sum;
    if !gas_hydraulic_mismatch_kg_m2_s.is_finite() {
        return Err(VegetationError::Domain(
            "V3 potential gas/hydraulic diagnostic",
        ));
    }

    Ok(OccupancyPassResult {
        candidate_state,
        liquid: final_liquid,
        local_layer_water_kg_m2_tile_ground,
        diagnostics: OccupancyDiagnostics {
            pass,
            ci_iterations_sun: accepted.canopy.sun.ci_iterations,
            ci_iterations_shade: accepted.canopy.shade.ci_iterations,
            energy_iterations: accepted.canopy.iterations,
            hydraulic_iterations: accepted.outer.iterations,
            outer_iterations: accepted.outer.iterations,
            normalized_residuals,
            temperature_step_k: accepted.canopy.temperature_step_k,
            potential_step_mm: Some(accepted.outer.potential_step_mm),
            backtracking_count: accepted
                .outer
                .backtracking_count
                .checked_add(accepted.canopy.backtracking_count)
                .ok_or(VegetationError::Domain(
                    "V3 occupancy diagnostic backtracking count",
                ))?,
            wet_store_cap_active: accepted.canopy.wet_store_cap_active,
            active_water_caps: accepted.outer.evaluation.active_water_caps.clone(),
            gas_hydraulic_mismatch_kg_m2_s,
            pivot_magnitude: Some(
                accepted
                    .outer
                    .pivot_magnitude
                    .min(accepted.canopy.pivot_magnitude.unwrap_or(f64::INFINITY)),
            )
            .filter(|value| value.is_finite()),
            matrix_norm: Some(
                accepted
                    .outer
                    .matrix_norm
                    .max(accepted.canopy.matrix_norm.unwrap_or(0.0)),
            ),
            advanced_t10_k: Some(advanced_t10_k),
            capped_operands: (pass == CoupledSolvePass::Capped)
                .then(|| {
                    crate::occupancy_solver::potential::capped_numerical_operands(
                        &accepted.outer.evaluation,
                        &accepted.outer.state,
                    )
                })
                .flatten(),
        },
    })
}

fn validate_identity(
    input: &OccupancyPassInput<'_>,
    radiation: &OccupancyRadiation,
    pass: CoupledSolvePass,
) -> Result<(), VegetationError> {
    match (pass, input.local_authorizations_kg_m2_tile_ground.as_ref()) {
        (CoupledSolvePass::Potential, None) | (CoupledSolvePass::Capped, Some(_)) => {}
        (CoupledSolvePass::Potential, Some(_)) => {
            return Err(VegetationError::Receipt(
                "owner authorization supplied during V3 potential pass".into(),
            ));
        }
        (CoupledSolvePass::Capped, None) => {
            return Err(VegetationError::Receipt(
                "owner authorization absent during V5 capped pass".into(),
            ));
        }
    }
    if !input.interval_s.is_finite() || input.interval_s <= 0.0 {
        return Err(VegetationError::Domain("V3 potential interval identity"));
    }
    if radiation.occupancy_id != *input.occupancy_id
        || input.stratum_config.stratum_id != input.occupancy_id.stratum_id
        || radiation.conditional_lai_m2_m2_tile_ground.to_bits()
            != input.conditional_lai_m2_m2_tile_ground.to_bits()
        || radiation.conditional_wai_m2_m2_tile_ground.to_bits()
            != input.conditional_wai_m2_m2_tile_ground.to_bits()
    {
        return Err(VegetationError::Receipt(
            "V3 production potential occupancy identity".into(),
        ));
    }
    Ok(())
}

fn local_cap_rates(
    input: &OccupancyPassInput<'_>,
) -> Result<BTreeMap<SoilLayerId, f64>, VegetationError> {
    let caps = input
        .local_authorizations_kg_m2_tile_ground
        .as_ref()
        .ok_or_else(|| VegetationError::Receipt("V5 capped local authorization absent".into()))?;
    caps.iter()
        .map(|(layer_id, amount)| {
            let rate = *amount / input.interval_s;
            if !rate.is_finite() || rate < 0.0 {
                return Err(VegetationError::Domain(
                    "V5 capped local authorization rate",
                ));
            }
            Ok((layer_id.clone(), rate))
        })
        .collect()
}

fn interception(
    input: &OccupancyPassInput<'_>,
    vapor_amount: f64,
    wet_surface_temperature_k: f64,
) -> Result<InterceptionResult, VegetationError> {
    liquid_interception(InterceptionInput {
        store0: input.occupancy_state.canopy_liquid_kg_h2o_m2_tile_ground,
        rain: input.incident_rain_kg_m2_tile_ground,
        vapor_amount,
        lai: input.conditional_lai_m2_m2_tile_ground,
        sai: input.conditional_wai_m2_m2_tile_ground,
        alpha_liq: input.stratum_config.alpha_liq,
        p_liq: input.stratum_config.p_liq_kg_m2_plant,
        stemflow_fraction: input.stratum_config.stemflow_fraction,
        leaf_temperature_k: wet_surface_temperature_k,
    })
}

#[allow(clippy::too_many_lines)]
fn prepare_case(
    input: &OccupancyPassInput<'_>,
    radiation: &OccupancyRadiation,
    preliminary: InterceptionResult,
    advanced_t10_k: f64,
) -> Result<V3PotentialCase, VegetationError> {
    let forcing = input.forcing;
    let config = input.stratum_config;
    let state = input.occupancy_state;
    let dt_s = input.interval_s;
    let class_areas = class_areas(forcing.direct_par_w_m2, forcing.direct_nir_w_m2, radiation)?;
    let leaf_n_area = accepted_leaf_n_area(input)?;
    let rd25 = if input.shared_state.leaf_area == 0.0 {
        0.0
    } else {
        atkin_rd25(
            accepted_leaf_nitrogen(input)?,
            input.shared_state.leaf_area,
            advanced_t10_k,
            config.atkin_intercept,
        )?
    };
    let sun = class_operands(class_areas.0, radiation, true, leaf_n_area, rd25, input)?;
    let shade = class_operands(class_areas.1, radiation, false, leaf_n_area, rd25, input)?;
    let u_star = canopy_surface_friction_velocity(
        forcing.wind_m_s,
        forcing.reference_height_m,
        config.displacement_m,
        config.z0m_m,
    )?;
    let gb_leaf = leaf_boundary_conductance(u_star, config.leaf_dimension_m)?;
    let gb_wet = leaf_boundary_conductance(u_star, config.wet_surface_dimension_m)?;
    let gb_stem = leaf_boundary_conductance(u_star, config.stem_dimension_m)?;
    let rah = neutral_resistance(
        forcing.reference_height_m,
        config.displacement_m,
        config.z0m_m,
        config.z0h_m,
        forcing.wind_m_s,
    )?;
    let raw = neutral_resistance(
        forcing.reference_height_m,
        config.displacement_m,
        config.z0m_m,
        config.z0q_m,
        forcing.wind_m_s,
    )?;

    Ok(V3PotentialCase {
        tile_fraction: input.tile_fraction,
        dt_s,
        gas_energy: GasEnergyOperands {
            pressure_pa: forcing.pressure_pa,
            ca_pa: forcing.co2_pa,
            derived_u_star_m_s: u_star,
            gb_leaf_m_s: gb_leaf,
            gb_wet_m_s: gb_wet,
            gb_stem_m_s: gb_stem,
            g0_umol_m2_s: config.g0_umol_h2o_m2_s,
            medlyn_g1_kpa_sqrt: config.g1_sqrt_kpa,
            cp_air_j_kg_k: CP_AIR_J_KG_K,
            latent_heat_j_kg: LATENT_HEAT_VAPORIZATION,
            rdry_j_kg_k: R_DRY_AIR_J_KG_K,
            air_temperature_k: forcing.air_temperature_k,
            air_specific_humidity_kg_kg: forcing.specific_humidity,
            reference_wind_operands: ReferenceWindOperands {
                kappa: VON_KARMAN,
                u_ref_m_s: forcing.wind_m_s,
                z_ref_m: forcing.reference_height_m,
                displacement_m: config.displacement_m,
                z0m_m: config.z0m_m,
                z0h_m: config.z0h_m,
                z0q_m: config.z0q_m,
            },
            rah_s_m: rah,
            raw_s_m: raw,
            leaf_emissivity: config.leaf_emissivity,
            wet_emissivity: config.wet_surface_emissivity,
            stem_emissivity: config.stem_emissivity,
            longwave_down_w_m2: forcing.longwave_down_w_m2,
            longwave_up_w_m2: forcing.longwave_up_w_m2,
            stem_area: input.conditional_wai_m2_m2_tile_ground,
            stem_absorbed_shortwave_w_m2_tile: stem_absorbed_shortwave(radiation),
            wet_fraction: preliminary.wet_fraction,
            canopy_liquid_kg_m2_tile: preliminary.store1,
            dt_s,
            wet_temperature_start_k: state.wet_surface_temperature_k,
            stem_temperature_start_k: state.dry_stem_temperature_k,
            canopy_air_temperature_start_k: state.canopy_air_temperature_k,
            qcan_start_kg_kg: state.canopy_air_specific_humidity_kg_kg,
        },
        classes: LeafClasses { sun, shade },
        biochemical_parameters: BiochemicalParameters {
            kc25_pa: config.kc25_pa,
            ko25_pa: config.ko25_pa,
            gamma25_pa: config.gamma25_pa,
            ha_vcmax_j_mol: config.ha_vcmax,
            hd_vcmax_j_mol: config.hd_vcmax,
            entropy_vcmax_j_mol_k: config.entropy_vcmax,
            ha_jmax_j_mol: config.ha_jmax,
            hd_jmax_j_mol: config.hd_jmax,
            entropy_jmax_j_mol_k: config.entropy_jmax,
            ha_kc_j_mol: config.ha_kc,
            ha_ko_j_mol: config.ha_ko,
            ha_gamma_j_mol: config.ha_gamma,
            tp_vcmax_ratio: config.tp_vcmax_ratio,
            oxygen_mole_fraction: OXYGEN_MOLE_FRACTION,
            electron_quantum_yield: ELECTRON_QUANTUM_YIELD,
            par_photon_umol_per_j: PAR_PHOTON_UMOL_PER_J,
            electron_curvature: ELECTRON_CURVATURE,
            ac_aj_curvature: AC_AJ_CURVATURE,
            ag_ap_curvature: AG_AP_CURVATURE,
        },
        parameters: HydraulicParameters {
            k1a_max_s1: config.k1a_max_s1,
            k1b_max_s1: config.k1b_max_s1,
            k2_max: config.k2_max_m_s,
            height_m: config.height_m,
            k3_max_m_s: config.k3_max_m_s,
            root_to_leaf_area: config.root_to_leaf_area,
            p50_root: config.p50_root_mm,
            p50_xylem: config.p50_stem_mm,
            p50_leaf: config.p50_leaf_mm,
            ck: config.vulnerability_shape,
        },
        layers: hydraulic_layers(input)?,
        surface_dimensions: SurfaceDimensions {
            leaf_m: config.leaf_dimension_m,
            wet_surface_m: config.wet_surface_dimension_m,
            stem_m: config.stem_dimension_m,
        },
    })
}

fn accepted_leaf_nitrogen(input: &OccupancyPassInput<'_>) -> Result<f64, VegetationError> {
    let pool = input
        .shared_state
        .tissues
        .get(&Tissue::Leaf)
        .ok_or(VegetationError::Domain("V4 displayed leaf nitrogen pool"))?;
    let nitrogen = pool.display.nitrogen;
    if nitrogen.is_finite() && nitrogen >= 0.0 {
        Ok(nitrogen)
    } else {
        Err(VegetationError::Domain("V4 displayed leaf nitrogen pool"))
    }
}

fn accepted_leaf_n_area(input: &OccupancyPassInput<'_>) -> Result<f64, VegetationError> {
    if input.shared_state.leaf_area == 0.0 {
        return if accepted_leaf_nitrogen(input)?.to_bits() == 0.0_f64.to_bits() {
            Ok(0.0)
        } else {
            Err(VegetationError::Domain(
                "V4 displayed leaf nitrogen without leaf area",
            ))
        };
    }
    let value = accepted_leaf_nitrogen(input)? / input.shared_state.leaf_area;
    if value.is_finite() && value >= 0.0 {
        Ok(value)
    } else {
        Err(VegetationError::Domain("V4 displayed leaf N area"))
    }
}

fn class_operands(
    leaf_area: f64,
    radiation: &OccupancyRadiation,
    sun: bool,
    leaf_n_area: f64,
    rd25: f64,
    input: &OccupancyPassInput<'_>,
) -> Result<LeafClassOperands, VegetationError> {
    let leaf_absorbed = |owner: &OwnedLayerAbsorption| {
        if sun {
            owner.absorbed_leaf_sun
        } else {
            owner.absorbed_leaf_shade
        }
    };
    let visible = leaf_absorbed(&radiation.visible_direct.absorption)
        + leaf_absorbed(&radiation.visible_diffuse.absorption);
    let shortwave = visible
        + leaf_absorbed(&radiation.near_infrared_direct.absorption)
        + leaf_absorbed(&radiation.near_infrared_diffuse.absorption);
    let temperature_start_k = if sun {
        input.occupancy_state.sun_leaf_temperature_k
    } else {
        input.occupancy_state.shade_leaf_temperature_k
    };
    if leaf_area == 0.0 {
        if visible != 0.0 || shortwave != 0.0 {
            return Err(VegetationError::Receipt(
                "V3 zero-area leaf radiation ownership".into(),
            ));
        }
        return Ok(LeafClassOperands {
            leaf_area: 0.0,
            absorbed_par_w_m2_leaf: 0.0,
            absorbed_shortwave_w_m2_tile: 0.0,
            vcmax25: 0.0,
            jmax25: 0.0,
            rd25: 0.0,
            temperature_start_k,
        });
    }
    let absorbed_par_w_m2_leaf = visible / leaf_area;
    let vcmax25 = leaf_n_area * input.stratum_config.rubisco_n_efficiency;
    let jmax25 = leaf_n_area * input.stratum_config.electron_n_efficiency;
    if !absorbed_par_w_m2_leaf.is_finite()
        || absorbed_par_w_m2_leaf < 0.0
        || !shortwave.is_finite()
        || shortwave < 0.0
        || [vcmax25, jmax25, rd25]
            .iter()
            .any(|value| !value.is_finite() || *value <= 0.0)
    {
        return Err(VegetationError::Domain("V3 active leaf-class operands"));
    }
    Ok(LeafClassOperands {
        leaf_area,
        absorbed_par_w_m2_leaf,
        absorbed_shortwave_w_m2_tile: shortwave,
        vcmax25,
        jmax25,
        rd25,
        temperature_start_k,
    })
}

fn class_areas(
    direct_par_w_m2: f64,
    direct_nir_w_m2: f64,
    radiation: &OccupancyRadiation,
) -> Result<(f64, f64), VegetationError> {
    let vis = &radiation.visible_direct.absorption;
    let nir = &radiation.near_infrared_direct.absorption;
    let selected = if direct_par_w_m2 > 0.0 {
        vis
    } else if direct_nir_w_m2 > 0.0 {
        nir
    } else {
        return Ok((0.0, radiation.conditional_lai_m2_m2_tile_ground));
    };
    if direct_par_w_m2 > 0.0
        && direct_nir_w_m2 > 0.0
        && (vis.leaf_sun_area.to_bits() != nir.leaf_sun_area.to_bits()
            || vis.leaf_shade_area.to_bits() != nir.leaf_shade_area.to_bits())
    {
        return Err(VegetationError::Receipt(
            "V3 direct-band leaf-area identity".into(),
        ));
    }
    let sum = selected.leaf_sun_area + selected.leaf_shade_area;
    let tolerance = 2.0e-12
        * sum
            .abs()
            .max(radiation.conditional_lai_m2_m2_tile_ground.abs())
            .max(1.0);
    if !sum.is_finite()
        || selected.leaf_sun_area < 0.0
        || selected.leaf_shade_area < 0.0
        || (sum - radiation.conditional_lai_m2_m2_tile_ground).abs() > tolerance
    {
        return Err(VegetationError::Receipt(
            "V3 sun/shade leaf-area closure".into(),
        ));
    }
    Ok((selected.leaf_sun_area, selected.leaf_shade_area))
}

fn stem_absorbed_shortwave(radiation: &OccupancyRadiation) -> f64 {
    radiation.visible_direct.absorption.absorbed_stem
        + radiation.visible_diffuse.absorption.absorbed_stem
        + radiation.near_infrared_direct.absorption.absorbed_stem
        + radiation.near_infrared_diffuse.absorption.absorbed_stem
}

fn hydraulic_layers(input: &OccupancyPassInput<'_>) -> Result<Vec<LayerOperands>, VegetationError> {
    let forcing_by_id = input
        .forcing
        .soil_layers
        .iter()
        .map(|layer| (&layer.layer_id, layer))
        .collect::<BTreeMap<_, _>>();
    input
        .stratum_config
        .root_layers
        .iter()
        .map(|root| {
            let forcing =
                forcing_by_id
                    .get(&root.layer_id)
                    .copied()
                    .ok_or(VegetationError::Domain(
                        "V3 hydraulic forcing layer identity",
                    ))?;
            layer_operands(root.root_fraction, root.lateral_root_length_m, forcing)
        })
        .collect()
}

fn layer_operands(
    root_fraction: f64,
    lateral_root_length_m: f64,
    forcing: &SoilLayerForcing,
) -> Result<LayerOperands, VegetationError> {
    let z3_m = forcing.root_path_length_mm / 1_000.0;
    let ksoil_m_s = forcing.hydraulic_conductivity_mm_s / 1_000.0;
    if !z3_m.is_finite() || z3_m <= 0.0 || !ksoil_m_s.is_finite() || ksoil_m_s < 0.0 {
        return Err(VegetationError::Domain("V3 hydraulic layer SI conversion"));
    }
    Ok(LayerOperands {
        layer_id: forcing.layer_id.as_str().to_owned(),
        soil_potential_mm: forcing.matric_potential_mm,
        gravity_head_mm: forcing.gravity_root_mm,
        root_fraction,
        z3_m,
        ksoil_m2_s: ksoil_m_s,
        dxroot_m: lateral_root_length_m,
        accessible: forcing.accessible,
        frozen: forcing.frozen,
    })
}

fn validate_layer_output(
    input: &OccupancyPassInput<'_>,
    actual: &[(SoilLayerId, f64)],
) -> Result<(), VegetationError> {
    if actual.len() != input.stratum_config.root_layers.len()
        || actual.iter().zip(&input.stratum_config.root_layers).any(
            |((actual_id, amount), expected)| {
                actual_id != &expected.layer_id || !amount.is_finite() || *amount < 0.0
            },
        )
    {
        return Err(VegetationError::Receipt(
            "V3 potential layer-demand identity".into(),
        ));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;

    use openwepp_kernel_contract::{ResourceOwnerId, SoilLayerId, TileId, TransactionId};

    use super::*;
    use crate::occupancy_solver::request_pass::execute_potential_column_pass;
    use crate::transaction::{
        CoupledOwnedState, SnowFreeForcing, SoilLayerForcing, v6_identity_rebound_fixture,
    };

    fn fixture() -> (VegetationConfiguration, CoupledOwnedState) {
        v6_identity_rebound_fixture()
    }

    fn forcing() -> SnowFreeForcing {
        SnowFreeForcing {
            air_temperature_k: 298.15,
            pressure_pa: 101_325.0,
            co2_pa: 42.0,
            vapor_pressure_deficit_kpa: 1.2,
            wind_m_s: 3.7,
            rain_kg_m2: 0.0,
            direct_par_w_m2: 410.0,
            diffuse_par_w_m2: 83.0,
            direct_nir_w_m2: 355.0,
            diffuse_nir_w_m2: 101.0,
            solar_zenith_cosine: 0.67,
            ground_albedo_vis: 0.14,
            ground_albedo_nir: 0.31,
            longwave_down_w_m2: 350.0,
            longwave_up_w_m2: 390.0,
            specific_humidity: 0.01,
            reference_height_m: 20.0,
            soil_layers: vec![SoilLayerForcing {
                layer_id: SoilLayerId::try_new("soil-1").expect("layer"),
                water_beginning_kg_m2: 20.0,
                matric_potential_mm: -1_000.0,
                hydraulic_conductivity_mm_s: 1.0e-5,
                root_path_length_mm: 100.0,
                gravity_root_mm: 500.0,
                temperature_k: 295.0,
                accessible: true,
                frozen: false,
            }],
            gsi: 1.0,
        }
    }

    fn owner() -> ResourceOwnerId {
        ResourceOwnerId::try_new("vegetation").expect("owner")
    }

    fn clear_absorption(absorption: &mut OwnedLayerAbsorption) {
        absorption.plant_area = 0.0;
        absorption.effective_reflectance = 0.0;
        absorption.effective_transmittance = 0.0;
        absorption.beam_extinction_unclumped = None;
        absorption.beam_extinction_effective = None;
        absorption.leaf_absorption_fraction = 0.0;
        absorption.stem_absorption_fraction = 0.0;
        absorption.absorbed_plant = 0.0;
        absorption.absorbed_leaf_sun = 0.0;
        absorption.absorbed_leaf_shade = 0.0;
        absorption.absorbed_stem = 0.0;
        absorption.leaf_sun_area = 0.0;
        absorption.leaf_shade_area = 0.0;
        absorption.owner_closure_residual = 0.0;
    }

    #[test]
    fn production_adapter_executes_real_uncapped_column_without_mutating_beginning() {
        let (configuration, state) = fixture();
        let beginning = serde_json::to_vec(&state).expect("beginning bytes");
        let evaluator = ProductionPotentialOccupancyEvaluator::from_configuration(&configuration)
            .expect("production evaluator");
        let tile = TileId::try_new("tile-1").expect("tile");
        let pass = execute_potential_column_pass(
            &configuration,
            &state,
            &forcing(),
            TransactionId(1),
            owner(),
            &BTreeMap::from([(tile, 0.0)]),
            &evaluator,
        )
        .expect("real V3 potential column");

        assert_eq!(pass.water_requests.requests().len(), 1);
        assert_eq!(pass.columns.columns.len(), 1);
        let occupancy = &pass.columns.columns[0].occupancy_results[0];
        assert_eq!(occupancy.stand_ground_layer_water_kg_m2.len(), 1);
        assert_eq!(occupancy.candidate_state.last_accepted_transaction_id, None);
        assert!(occupancy.diagnostics.energy_iterations > 0);
        assert_eq!(
            occupancy.diagnostics.pass,
            crate::diagnostics::CoupledSolvePass::Potential
        );
        let expected_t10 = update_t10(
            state.strata.values().next().expect("shared").t10_k,
            forcing().air_temperature_k,
            configuration.dt_s,
        )
        .expect("advanced T10");
        assert_eq!(occupancy.diagnostics.advanced_t10_k, Some(expected_t10));
        assert!(
            occupancy
                .diagnostics
                .normalized_residuals
                .iter()
                .all(|residual| residual.value.is_finite())
        );
        assert_eq!(serde_json::to_vec(&state).expect("after bytes"), beginning);
    }

    #[test]
    fn production_adapter_accepts_exact_zero_plant_nested_energy_state() {
        let (configuration, state) = fixture();
        let forcing = forcing();
        let occupancy_id = configuration
            .expected_occupancies()
            .into_iter()
            .next()
            .expect("occupancy");
        let mut shared = state.strata[&occupancy_id.stratum_id].clone();
        shared.leaf_area = 0.0;
        shared.stem_area = 0.0;
        shared.root_area = 0.0;
        *shared.tissues.get_mut(&Tissue::Leaf).expect("leaf pool") =
            crate::carbon_nitrogen::TissuePool::default();
        let lane = &state.occupancies[&occupancy_id];
        let mut radiation = crate::occupancy_solver::radiation::prepare_whole_column_radiation(
            &configuration,
            &state,
            &forcing,
        )
        .expect("prepared radiation")
        .occupancies[&occupancy_id]
            .clone();
        radiation.conditional_lai_m2_m2_tile_ground = 0.0;
        radiation.conditional_wai_m2_m2_tile_ground = 0.0;
        clear_absorption(&mut radiation.visible_direct.absorption);
        clear_absorption(&mut radiation.visible_diffuse.absorption);
        clear_absorption(&mut radiation.near_infrared_direct.absorption);
        clear_absorption(&mut radiation.near_infrared_diffuse.absorption);
        let input = OccupancyPassInput {
            transaction_id: TransactionId(1),
            interval_s: configuration.dt_s,
            occupancy_id: &occupancy_id,
            tile_fraction: 1.0,
            coverage: 1.0,
            conditional_lai_m2_m2_tile_ground: 0.0,
            conditional_wai_m2_m2_tile_ground: 0.0,
            incident_rain_kg_m2_tile_ground: 0.0,
            local_authorizations_kg_m2_tile_ground: None,
            shared_state: &shared,
            occupancy_state: lane,
            stratum_config: &configuration.strata[0],
            forcing: &forcing,
        };
        let evaluator = ProductionPotentialOccupancyEvaluator::from_configuration(&configuration)
            .expect("production evaluator");
        let accepted = evaluator
            .solve_potential(input, &radiation)
            .expect("zero-plant production adapter path");

        assert!(
            accepted
                .local_layer_water_kg_m2_tile_ground
                .iter()
                .all(|(_, amount)| amount.to_bits() == 0.0_f64.to_bits())
        );
        assert_eq!(
            accepted
                .diagnostics
                .gas_hydraulic_mismatch_kg_m2_s
                .to_bits(),
            0.0_f64.to_bits()
        );
        assert_eq!(
            accepted.candidate_state.beta_hyd.to_bits(),
            1.0_f64.to_bits()
        );
        assert!(accepted.diagnostics.energy_iterations > 0);
    }

    #[test]
    fn stale_t10_is_not_used_and_ci_feedback_does_not_change_fixed_brent_endpoint() {
        let (configuration, mut state) = fixture();
        state.last_transaction_id = 1;
        for shared in state.strata.values_mut() {
            shared.last_transaction_id = 1;
        }
        for lane in state.occupancies.values_mut() {
            lane.last_accepted_transaction_id = Some(1);
        }
        state.state_sha256 = state.canonical_sha256().expect("accepted state digest");
        let occupancy_id = configuration
            .expected_occupancies()
            .into_iter()
            .next()
            .expect("occupancy");
        let evaluator = ProductionPotentialOccupancyEvaluator::from_configuration(&configuration)
            .expect("production evaluator");
        let tile = TileId::try_new("tile-1").expect("tile");
        let rain = BTreeMap::from([(tile, 0.0)]);
        let run = |candidate: &CoupledOwnedState| {
            execute_potential_column_pass(
                &configuration,
                candidate,
                &forcing(),
                TransactionId(2),
                owner(),
                &rain,
                &evaluator,
            )
            .expect("V3 potential pass")
        };
        let baseline = run(&state);
        let baseline_occupancy = &baseline.columns.columns[0].occupancy_results[0];
        let expected_t10 = update_t10(
            state.strata[&occupancy_id.stratum_id].t10_k,
            forcing().air_temperature_k,
            configuration.dt_s,
        )
        .expect("advanced T10");
        assert_eq!(
            baseline_occupancy.diagnostics.advanced_t10_k,
            Some(expected_t10)
        );
        assert_ne!(
            expected_t10.to_bits(),
            state.strata[&occupancy_id.stratum_id].t10_k.to_bits()
        );

        let lane = state
            .occupancies
            .get_mut(&occupancy_id)
            .expect("occupancy lane");
        lane.sun_ci_pa = 6.0;
        lane.shade_ci_pa = 40.0;
        state.state_sha256 = state.canonical_sha256().expect("mutated state digest");
        let varied = run(&state);
        let varied_occupancy = &varied.columns.columns[0].occupancy_results[0];
        assert_eq!(
            varied_occupancy.candidate_state.sun_ci_pa.to_bits(),
            baseline_occupancy.candidate_state.sun_ci_pa.to_bits()
        );
        assert_eq!(
            varied_occupancy.candidate_state.shade_ci_pa.to_bits(),
            baseline_occupancy.candidate_state.shade_ci_pa.to_bits()
        );
        assert_eq!(
            varied_occupancy.diagnostics.ci_iterations_sun,
            baseline_occupancy.diagnostics.ci_iterations_sun
        );
        assert_eq!(
            varied_occupancy.diagnostics.ci_iterations_shade,
            baseline_occupancy.diagnostics.ci_iterations_shade
        );
    }

    #[test]
    fn adapter_maps_authoritative_runtime_operands_without_fixture_values() {
        let (configuration, state) = fixture();
        let forcing = forcing();
        let prepared = crate::occupancy_solver::radiation::prepare_whole_column_radiation(
            &configuration,
            &state,
            &forcing,
        )
        .expect("prepared radiation");
        let occupancy_id = configuration
            .expected_occupancies()
            .into_iter()
            .next()
            .expect("occupancy");
        let mut shared = state.strata[&occupancy_id.stratum_id].clone();
        let leaf = shared.tissues.get_mut(&Tissue::Leaf).expect("leaf pool");
        leaf.storage.nitrogen = 9.0;
        leaf.transfer.nitrogen = 11.0;
        let lane = &state.occupancies[&occupancy_id];
        let radiation = &prepared.occupancies[&occupancy_id];
        let stratum = &configuration.strata[0];
        let input = OccupancyPassInput {
            transaction_id: TransactionId(1),
            interval_s: configuration.dt_s,
            occupancy_id: &occupancy_id,
            tile_fraction: configuration.topology_tiles[0].fraction,
            coverage: 1.0,
            conditional_lai_m2_m2_tile_ground: shared.leaf_area,
            conditional_wai_m2_m2_tile_ground: shared.stem_area,
            incident_rain_kg_m2_tile_ground: 0.0,
            local_authorizations_kg_m2_tile_ground: None,
            shared_state: &shared,
            occupancy_state: lane,
            stratum_config: stratum,
            forcing: &forcing,
        };
        let preliminary =
            interception(&input, 0.0, lane.wet_surface_temperature_k).expect("preliminary E04");
        let advanced_t10 = update_t10(shared.t10_k, forcing.air_temperature_k, configuration.dt_s)
            .expect("advanced T10");
        let case = prepare_case(&input, radiation, preliminary, advanced_t10)
            .expect("runtime constitutive case");

        let leaf_n_area = 0.003 / shared.leaf_area;
        assert_eq!(
            case.classes.sun.vcmax25.to_bits(),
            (leaf_n_area * stratum.rubisco_n_efficiency).to_bits()
        );
        assert_eq!(
            case.classes.shade.jmax25.to_bits(),
            (leaf_n_area * stratum.electron_n_efficiency).to_bits()
        );
        let expected_rd25 = atkin_rd25(
            0.003,
            shared.leaf_area,
            advanced_t10,
            stratum.atkin_intercept,
        )
        .expect("Atkin Rd25");
        assert_eq!(case.classes.sun.rd25.to_bits(), expected_rd25.to_bits());
        assert_eq!(case.classes.shade.rd25.to_bits(), expected_rd25.to_bits());
        assert_eq!(case.dt_s.to_bits(), configuration.dt_s.to_bits());
        assert_eq!(
            case.gas_energy.canopy_liquid_kg_m2_tile.to_bits(),
            preliminary.store1.to_bits()
        );
        assert_eq!(
            case.gas_energy.wet_fraction.to_bits(),
            preliminary.wet_fraction.to_bits()
        );
        assert_eq!(case.layers.len(), 1);
        assert_eq!(
            case.layers[0].z3_m.to_bits(),
            (100.0_f64 / 1_000.0).to_bits()
        );
        assert_eq!(
            case.layers[0].ksoil_m2_s.to_bits(),
            (1.0e-5_f64 / 1_000.0).to_bits()
        );
        assert_eq!(
            case.layers[0].dxroot_m.to_bits(),
            stratum.root_layers[0].lateral_root_length_m.to_bits()
        );
        assert_eq!(
            case.classes.sun.leaf_area.to_bits(),
            radiation.visible_direct.absorption.leaf_sun_area.to_bits()
        );
        assert_eq!(
            case.classes.shade.leaf_area.to_bits(),
            radiation
                .visible_direct
                .absorption
                .leaf_shade_area
                .to_bits()
        );
    }

    #[test]
    fn v4_leaf_capacity_uses_displayed_nitrogen_only_and_rejects_display_n_without_lai() {
        let (configuration, state) = fixture();
        let forcing = forcing();
        let occupancy_id = configuration
            .expected_occupancies()
            .into_iter()
            .next()
            .expect("occupancy");
        let lane = &state.occupancies[&occupancy_id];
        let stratum = &configuration.strata[0];
        let mut shared = state.strata[&occupancy_id.stratum_id].clone();
        let leaf = shared.tissues.get_mut(&Tissue::Leaf).expect("leaf pool");
        leaf.storage.nitrogen = 9.0;
        leaf.transfer.nitrogen = 11.0;
        {
            let input = OccupancyPassInput {
                transaction_id: TransactionId(1),
                interval_s: configuration.dt_s,
                occupancy_id: &occupancy_id,
                tile_fraction: 1.0,
                coverage: 1.0,
                conditional_lai_m2_m2_tile_ground: shared.leaf_area,
                conditional_wai_m2_m2_tile_ground: shared.stem_area,
                incident_rain_kg_m2_tile_ground: 0.0,
                local_authorizations_kg_m2_tile_ground: None,
                shared_state: &shared,
                occupancy_state: lane,
                stratum_config: stratum,
                forcing: &forcing,
            };
            assert_eq!(accepted_leaf_nitrogen(&input), Ok(0.003));
            assert_eq!(
                accepted_leaf_n_area(&input)
                    .expect("displayed leaf N area")
                    .to_bits(),
                (0.003_f64 / shared.leaf_area).to_bits()
            );
        }

        let mut zero_shared = shared.clone();
        zero_shared.leaf_area = 0.0;
        zero_shared
            .tissues
            .get_mut(&Tissue::Leaf)
            .expect("leaf pool")
            .display
            .nitrogen = 0.0;
        {
            let zero_input = OccupancyPassInput {
                transaction_id: TransactionId(1),
                interval_s: configuration.dt_s,
                occupancy_id: &occupancy_id,
                tile_fraction: 1.0,
                coverage: 1.0,
                conditional_lai_m2_m2_tile_ground: 0.0,
                conditional_wai_m2_m2_tile_ground: zero_shared.stem_area,
                incident_rain_kg_m2_tile_ground: 0.0,
                local_authorizations_kg_m2_tile_ground: None,
                shared_state: &zero_shared,
                occupancy_state: lane,
                stratum_config: stratum,
                forcing: &forcing,
            };
            assert_eq!(accepted_leaf_n_area(&zero_input), Ok(0.0));
        }

        zero_shared
            .tissues
            .get_mut(&Tissue::Leaf)
            .expect("leaf pool")
            .display
            .nitrogen = f64::MIN_POSITIVE;
        let invalid_input = OccupancyPassInput {
            transaction_id: TransactionId(1),
            interval_s: configuration.dt_s,
            occupancy_id: &occupancy_id,
            tile_fraction: 1.0,
            coverage: 1.0,
            conditional_lai_m2_m2_tile_ground: 0.0,
            conditional_wai_m2_m2_tile_ground: zero_shared.stem_area,
            incident_rain_kg_m2_tile_ground: 0.0,
            local_authorizations_kg_m2_tile_ground: None,
            shared_state: &zero_shared,
            occupancy_state: lane,
            stratum_config: stratum,
            forcing: &forcing,
        };
        assert_eq!(
            accepted_leaf_n_area(&invalid_input),
            Err(VegetationError::Domain(
                "V4 displayed leaf nitrogen without leaf area"
            ))
        );
    }

    #[test]
    fn adapter_rd_mapping_matches_independent_v3_fixture() {
        let (configuration, state) = fixture();
        let mut forcing = forcing();
        forcing.air_temperature_k = 293.15;
        let prepared = crate::occupancy_solver::radiation::prepare_whole_column_radiation(
            &configuration,
            &state,
            &forcing,
        )
        .expect("prepared radiation");
        let occupancy_id = configuration
            .expected_occupancies()
            .into_iter()
            .next()
            .expect("occupancy");
        let mut shared = state.strata[&occupancy_id.stratum_id].clone();
        shared.t10_k = 293.15;
        let leaf = shared.tissues.get_mut(&Tissue::Leaf).expect("leaf pool");
        leaf.display.nitrogen = 0.004;
        leaf.storage.nitrogen = 0.0;
        leaf.transfer.nitrogen = 0.0;
        let mut stratum = configuration.strata[0].clone();
        stratum.atkin_intercept = 0.82;
        let lane = &state.occupancies[&occupancy_id];
        let radiation = &prepared.occupancies[&occupancy_id];
        let input = OccupancyPassInput {
            transaction_id: TransactionId(1),
            interval_s: 1_800.0,
            occupancy_id: &occupancy_id,
            tile_fraction: 1.0,
            coverage: 1.0,
            conditional_lai_m2_m2_tile_ground: shared.leaf_area,
            conditional_wai_m2_m2_tile_ground: shared.stem_area,
            incident_rain_kg_m2_tile_ground: 0.0,
            local_authorizations_kg_m2_tile_ground: None,
            shared_state: &shared,
            occupancy_state: lane,
            stratum_config: &stratum,
            forcing: &forcing,
        };
        let preliminary =
            interception(&input, 0.0, lane.wet_surface_temperature_k).expect("preliminary E04");
        let advanced_t10 = update_t10(shared.t10_k, forcing.air_temperature_k, input.interval_s)
            .expect("advanced T10");
        let case = prepare_case(&input, radiation, preliminary, advanced_t10)
            .expect("runtime constitutive case");
        let vectors: serde_json::Value = serde_json::from_str(include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../docs/work-packages/20260812-c3-woody-potential-pass-authority-001/artifacts/openwepp_c3_woody_v3_vectors.json"
        )))
        .expect("independent V3 vectors");
        let expected =
            vectors["families"]["leaf_respiration"]["results"]["rd25_umol_co2_m2_leaf_s"]
                .as_f64()
                .expect("independent Rd25");
        assert!((case.classes.sun.rd25 - expected).abs() <= f64::EPSILON);
        assert!((case.classes.shade.rd25 - expected).abs() <= f64::EPSILON);
    }

    #[test]
    fn production_adapter_rejects_wrong_radiation_identity_before_state_change() {
        let (configuration, state) = fixture();
        let beginning = serde_json::to_vec(&state).expect("beginning bytes");
        let forcing = forcing();
        let prepared = crate::occupancy_solver::radiation::prepare_whole_column_radiation(
            &configuration,
            &state,
            &forcing,
        )
        .expect("radiation");
        let occupancy_id = configuration
            .expected_occupancies()
            .into_iter()
            .next()
            .expect("occupancy");
        let mut radiation = prepared
            .occupancies
            .get(&occupancy_id)
            .expect("occupancy radiation")
            .clone();
        radiation.occupancy_id.tile_id = TileId::try_new("wrong-tile").expect("wrong tile");
        let shared = state.strata.get(&occupancy_id.stratum_id).expect("shared");
        let lane = state.occupancies.get(&occupancy_id).expect("lane");
        let stratum = &configuration.strata[0];
        let input = OccupancyPassInput {
            transaction_id: TransactionId(1),
            interval_s: configuration.dt_s,
            occupancy_id: &occupancy_id,
            tile_fraction: 1.0,
            coverage: 1.0,
            conditional_lai_m2_m2_tile_ground: shared.leaf_area,
            conditional_wai_m2_m2_tile_ground: shared.stem_area,
            incident_rain_kg_m2_tile_ground: 0.0,
            local_authorizations_kg_m2_tile_ground: None,
            shared_state: shared,
            occupancy_state: lane,
            stratum_config: stratum,
            forcing: &forcing,
        };
        let evaluator = ProductionPotentialOccupancyEvaluator::from_configuration(&configuration)
            .expect("production evaluator");
        assert!(matches!(
            evaluator.solve_potential(input, &radiation),
            Err(VegetationError::Receipt(_))
        ));
        assert_eq!(serde_json::to_vec(&state).expect("after bytes"), beginning);
    }
}
