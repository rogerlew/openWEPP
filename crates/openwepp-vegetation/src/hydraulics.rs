//! E14--E15 four-potential plant hydraulics and authorization complementarity.

use crate::VegetationError;
use crate::numerics::{NewtonSystem, SolverDiagnostics, damped_newton};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct HydraulicLayerInput {
    pub soil_psi_mm: f64,
    pub root_fraction: f64,
    pub soil_conductivity_mm_s: f64,
    pub root_path_length_mm: f64,
    pub lateral_root_length_mm: f64,
    pub gravity_root_mm: f64,
    pub accessible: bool,
    pub frozen: bool,
}

#[derive(Clone, Debug, PartialEq)]
pub struct HydraulicInput {
    pub dt_s: f64,
    pub lai_sun: f64,
    pub lai_shade: f64,
    pub sai: f64,
    pub emax_sun_mm_s: f64,
    pub emax_shade_mm_s: f64,
    pub k1a_max_s1: f64,
    pub k1b_max_s1: f64,
    pub k2_max_mm_s: f64,
    pub k3_max_mm_s: f64,
    pub stem_path_length_mm: f64,
    pub root_to_leaf_area: f64,
    pub gravity_stem_mm: f64,
    pub p50_leaf_mm: f64,
    pub p50_stem_mm: f64,
    pub p50_root_mm: f64,
    pub shape: f64,
    pub layers: Vec<HydraulicLayerInput>,
    pub warm_start: [f64; 4],
}

#[derive(Clone, Debug, PartialEq)]
pub struct HydraulicSolution {
    pub psi_sun_mm: f64,
    pub psi_shade_mm: f64,
    pub psi_stem_mm: f64,
    pub psi_root_mm: f64,
    pub layer_law_flux_mm_s: Vec<f64>,
    pub finalized_amounts_kg_m2: Vec<f64>,
    pub transpiration_mm_s: f64,
    pub residuals: Vec<f64>,
    pub active_caps: Vec<usize>,
    pub diagnostics: SolverDiagnostics,
}

pub fn vulnerability(psi: f64, p50: f64, shape: f64) -> Result<f64, VegetationError> {
    if ![psi, p50, shape].iter().all(|v| v.is_finite()) || p50 >= 0.0 || shape <= 0.0 {
        return Err(VegetationError::Domain("vulnerability"));
    }
    let value = 2.0_f64.powf(-(psi / p50).powf(shape));
    if value.is_finite() {
        Ok(value)
    } else {
        Err(VegetationError::Domain("vulnerability overflow"))
    }
}

pub fn solve_hydraulics(
    input: &HydraulicInput,
    authorization_amounts: Option<&[f64]>,
) -> Result<HydraulicSolution, VegetationError> {
    validate(input, authorization_amounts)?;
    let n = input.layers.len();
    let mut initial = input.warm_start.to_vec();
    for layer in &input.layers {
        let law = layer_law(input, layer, input.warm_start[3])?;
        initial.push(if layer.accessible && !layer.frozen {
            law
        } else {
            0.0
        });
    }
    let unit_scales = (0..4 + n)
        .map(|index| if index < 4 { 1.0 } else { 1e-6 })
        .collect::<Vec<_>>();
    let mut atols = vec![1e-12; 4 + n];
    for value in &mut atols[4..] {
        *value = 1e-12;
    }
    let demand_scale = (input.emax_sun_mm_s + input.emax_shade_mm_s).max(1e-15);
    let mut physical_scales = vec![demand_scale; 4 + n];
    for (index, layer) in input.layers.iter().enumerate() {
        physical_scales[4 + index] = layer_law(input, layer, input.warm_start[3])?
            .abs()
            .max(1e-15);
    }
    let residual = |state: &[f64]| hydraulic_residual(input, authorization_amounts, state);
    let (state, diagnostics) = damped_newton(
        NewtonSystem::Hydraulic,
        &initial,
        &unit_scales,
        &atols,
        &physical_scales,
        1e-9,
        1e-7,
        50,
        20,
        residual,
    )?;
    let residuals = hydraulic_residual(input, authorization_amounts, &state)?;
    let mut laws = Vec::with_capacity(n);
    let mut amounts = Vec::with_capacity(n);
    let mut active = Vec::new();
    for (index, layer) in input.layers.iter().enumerate() {
        let law = if layer.accessible && !layer.frozen {
            layer_law(input, layer, state[3])?
        } else {
            0.0
        };
        if law < 0.0 {
            return Err(VegetationError::Hydraulic(
                "hydraulic redistribution unsupported",
            ));
        }
        laws.push(law);
        amounts.push(state[4 + index] * input.dt_s);
        if authorization_amounts
            .is_some_and(|caps| state[4 + index] * input.dt_s >= caps[index] - 1e-12)
        {
            active.push(index);
        }
    }
    Ok(HydraulicSolution {
        psi_sun_mm: state[0],
        psi_shade_mm: state[1],
        psi_stem_mm: state[2],
        psi_root_mm: state[3],
        layer_law_flux_mm_s: laws,
        finalized_amounts_kg_m2: amounts,
        transpiration_mm_s: state[4..].iter().sum(),
        residuals,
        active_caps: active,
        diagnostics,
    })
}

fn hydraulic_residual(
    input: &HydraulicInput,
    caps: Option<&[f64]>,
    state: &[f64],
) -> Result<Vec<f64>, VegetationError> {
    if state.len() != 4 + input.layers.len() || state.iter().any(|v| !v.is_finite()) {
        return Err(VegetationError::Domain("hydraulic state"));
    }
    let sun = state[0];
    let shade = state[1];
    let stem = state[2];
    let root = state[3];
    let e_sun = input.emax_sun_mm_s * vulnerability(sun, input.p50_leaf_mm, input.shape)?;
    let e_shade = input.emax_shade_mm_s * vulnerability(shade, input.p50_leaf_mm, input.shape)?;
    let root_v = vulnerability(root, input.p50_stem_mm, input.shape)?;
    let stem_v = vulnerability(stem, input.p50_leaf_mm, input.shape)?;
    let q1a = input.k1a_max_s1 * stem_v * input.lai_sun * (stem - sun);
    let q1b = input.k1b_max_s1 * stem_v * input.lai_shade * (stem - shade);
    let q2 = input.k2_max_mm_s / input.stem_path_length_mm
        * root_v
        * input.sai
        * (root - stem - input.gravity_stem_mm);
    let mut out = vec![
        e_sun - q1a,
        e_shade - q1b,
        q1a + q1b - q2,
        q2 - state[4..].iter().sum::<f64>(),
    ];
    for (index, layer) in input.layers.iter().enumerate() {
        let law = if layer.accessible && !layer.frozen && layer.root_fraction > 0.0 {
            layer_law(input, layer, root)?
        } else {
            0.0
        };
        if law < 0.0 {
            return Err(VegetationError::Hydraulic(
                "hydraulic redistribution unsupported",
            ));
        }
        let maximum = caps.map_or(law, |values| (values[index] / input.dt_s).min(law));
        out.push(state[4 + index] - maximum);
    }
    Ok(out)
}

fn layer_law(
    input: &HydraulicInput,
    layer: &HydraulicLayerInput,
    root: f64,
) -> Result<f64, VegetationError> {
    let rai = (input.lai_sun + input.lai_shade + input.sai)
        * layer.root_fraction
        * input.root_to_leaf_area;
    let kr = input.k3_max_mm_s / layer.root_path_length_mm
        * vulnerability(layer.soil_psi_mm, input.p50_root_mm, input.shape)?;
    let ks = layer.soil_conductivity_mm_s / layer.lateral_root_length_mm;
    let series = kr * ks / (kr + ks);
    let value = series * rai * (layer.soil_psi_mm - root + layer.gravity_root_mm);
    if value.is_finite() {
        Ok(value)
    } else {
        Err(VegetationError::Domain("layer hydraulic flux"))
    }
}

fn validate(input: &HydraulicInput, caps: Option<&[f64]>) -> Result<(), VegetationError> {
    let values = [
        input.dt_s,
        input.lai_sun,
        input.lai_shade,
        input.sai,
        input.emax_sun_mm_s,
        input.emax_shade_mm_s,
        input.k1a_max_s1,
        input.k1b_max_s1,
        input.k2_max_mm_s,
        input.k3_max_mm_s,
        input.stem_path_length_mm,
        input.root_to_leaf_area,
        input.gravity_stem_mm,
        input.p50_leaf_mm,
        input.p50_stem_mm,
        input.p50_root_mm,
        input.shape,
    ];
    if values.iter().any(|v| !v.is_finite())
        || input.dt_s <= 0.0
        || input.lai_sun < 0.0
        || input.lai_shade < 0.0
        || input.sai < 0.0
        || input.emax_sun_mm_s < 0.0
        || input.emax_shade_mm_s < 0.0
        || input.k1a_max_s1 <= 0.0
        || input.k1b_max_s1 <= 0.0
        || input.k2_max_mm_s <= 0.0
        || input.k3_max_mm_s <= 0.0
        || input.stem_path_length_mm <= 0.0
        || input.root_to_leaf_area <= 0.0
        || input.p50_leaf_mm >= 0.0
        || input.p50_stem_mm >= 0.0
        || input.p50_root_mm >= 0.0
        || input.shape <= 0.0
    {
        return Err(VegetationError::Domain("hydraulic input"));
    }
    if let Some(values) = caps {
        if values.len() != input.layers.len() || values.iter().any(|v| !v.is_finite() || *v < 0.0) {
            return Err(VegetationError::Hydraulic("authorization shape/domain"));
        }
    }
    let sum = input.layers.iter().map(|l| l.root_fraction).sum::<f64>();
    if (sum - 1.0).abs() > 1e-12 {
        return Err(VegetationError::Domain("hydraulic root fractions"));
    }
    for layer in &input.layers {
        if ![
            layer.soil_psi_mm,
            layer.root_fraction,
            layer.soil_conductivity_mm_s,
            layer.root_path_length_mm,
            layer.lateral_root_length_mm,
            layer.gravity_root_mm,
        ]
        .iter()
        .all(|v| v.is_finite())
            || layer.root_fraction < 0.0
            || layer.soil_conductivity_mm_s < 0.0
            || layer.root_path_length_mm <= 0.0
            || layer.lateral_root_length_mm <= 0.0
        {
            return Err(VegetationError::Domain("hydraulic layer"));
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn oracle_input(dt_s: f64) -> HydraulicInput {
        HydraulicInput {
            dt_s,
            lai_sun: 1.6,
            lai_shade: 1.2,
            sai: 0.7,
            emax_sun_mm_s: 0.58 * 3.0e-5,
            emax_shade_mm_s: 0.42 * 3.0e-5,
            k1a_max_s1: 4.0e-5,
            k1b_max_s1: 3.5e-5,
            k2_max_mm_s: 2.8e-5,
            k3_max_mm_s: 2.2e-5,
            stem_path_length_mm: 1.0,
            root_to_leaf_area: 1.0,
            gravity_stem_mm: 980.0,
            p50_leaf_mm: -120_000.0,
            p50_stem_mm: -160_000.0,
            p50_root_mm: -140_000.0,
            shape: 2.0,
            layers: vec![
                HydraulicLayerInput {
                    soil_psi_mm: -25_000.0,
                    root_fraction: 0.35,
                    soil_conductivity_mm_s: 1.7e-5,
                    root_path_length_mm: 1.0,
                    lateral_root_length_mm: 1.0,
                    gravity_root_mm: 980.0,
                    accessible: true,
                    frozen: false,
                },
                HydraulicLayerInput {
                    soil_psi_mm: -25_000.0,
                    root_fraction: 0.65,
                    soil_conductivity_mm_s: 1.7e-5,
                    root_path_length_mm: 1.0,
                    lateral_root_length_mm: 1.0,
                    gravity_root_mm: 980.0,
                    accessible: true,
                    frozen: false,
                },
            ],
            warm_start: [-33_000.0, -32_000.0, -28_500.0, -25_500.0],
        }
    }

    #[test]
    fn four_node_and_cap_resolve_match_independent_oracle() {
        let full = solve_hydraulics(&oracle_input(1.0), None).expect("four-node solution");
        assert!((full.psi_root_mm - -24_020.875_766_351_45).abs() < 1e-5);
        assert!((full.finalized_amounts_kg_m2[0] - 1.018_875_162_279_581e-5).abs() < 1e-12);
        assert!((full.finalized_amounts_kg_m2[1] - 1.892_196_729_947_793_4e-5).abs() < 1e-12);
        let limited = solve_hydraulics(&oracle_input(1_800.0), Some(&[0.0144, 0.0234]))
            .expect("cap-active solution");
        assert_eq!(limited.active_caps, vec![0, 1]);
        assert!((limited.finalized_amounts_kg_m2[0] - 0.0144).abs() < 1e-12);
        assert!((limited.finalized_amounts_kg_m2[1] - 0.0234).abs() < 1e-12);
        let mut redistribution = oracle_input(1.0);
        redistribution.layers[0].soil_psi_mm = -1_000.0;
        redistribution.layers[1].soil_psi_mm = -1_000_000.0;
        assert!(solve_hydraulics(&redistribution, None).is_err());
    }
}
