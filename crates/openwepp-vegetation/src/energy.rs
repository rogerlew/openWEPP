//! E06/E13 neutral aerodynamics, saturation humidity, and energy closure.
use crate::VegetationError;
use crate::numerics::{NewtonSystem, SolverDiagnostics, damped_newton};
use crate::photosynthesis::{CiSolution, FvcbInput, arrhenius, peaked_response, solve_ci};

pub const STEFAN_BOLTZMANN: f64 = 5.670_374_419e-8;
pub const LATENT_HEAT_VAPORIZATION: f64 = 2_501_000.0;
pub const VON_KARMAN: f64 = 0.4;
const ESAT_HPA: [f64; 9] = [
    6.112_134_76,
    0.444_007_856,
    0.014_306_423_4,
    0.000_264_461_437,
    0.000_003_059_035_58,
    1.962_372_41e-8,
    8.923_447_72e-11,
    -3.732_084_10e-13,
    2.093_399_97e-16,
];

pub fn neutral_resistance(
    z_ref: f64,
    displacement: f64,
    z0_a: f64,
    z0_b: f64,
    wind: f64,
) -> Result<f64, VegetationError> {
    if [z_ref, displacement, z0_a, z0_b, wind]
        .iter()
        .any(|v| !v.is_finite())
        || z0_a <= 0.0
        || z0_b <= 0.0
        || wind <= 0.0
        || z_ref <= displacement + z0_a.max(z0_b)
    {
        return Err(VegetationError::Unsupported(
            "CALM_OR_NONNEUTRAL_AERODYNAMICS",
        ));
    }
    Ok(
        ((z_ref - displacement) / z0_a).ln() * ((z_ref - displacement) / z0_b).ln()
            / (0.4_f64.powi(2) * wind),
    )
}

/// Derives the V3 canopy-surface semantic wind from reference wind.
pub fn canopy_surface_friction_velocity(
    reference_wind_m_s: f64,
    reference_height_m: f64,
    displacement_m: f64,
    momentum_roughness_m: f64,
) -> Result<f64, VegetationError> {
    if [
        reference_wind_m_s,
        reference_height_m,
        displacement_m,
        momentum_roughness_m,
    ]
    .iter()
    .any(|value| !value.is_finite())
        || reference_wind_m_s <= 0.0
        || momentum_roughness_m <= 0.0
        || reference_height_m <= displacement_m + momentum_roughness_m
    {
        return Err(VegetationError::Unsupported(
            "CALM_OR_NONNEUTRAL_AERODYNAMICS",
        ));
    }
    let u_star = VON_KARMAN * reference_wind_m_s
        / ((reference_height_m - displacement_m) / momentum_roughness_m).ln();
    if u_star.is_finite() && u_star > 0.0 {
        Ok(u_star)
    } else {
        Err(VegetationError::Unsupported(
            "CALM_OR_NONNEUTRAL_AERODYNAMICS",
        ))
    }
}

pub fn leaf_boundary_conductance(wind_leaf: f64, dimension: f64) -> Result<f64, VegetationError> {
    if !wind_leaf.is_finite() || !dimension.is_finite() || wind_leaf <= 0.0 || dimension <= 0.0 {
        return Err(VegetationError::Domain("leaf boundary conductance"));
    }
    Ok(0.01 * (wind_leaf / dimension).sqrt())
}

pub fn saturation_vapor_pressure_pa(temperature_k: f64) -> Result<f64, VegetationError> {
    if !temperature_k.is_finite() || !(273.15..=373.15).contains(&temperature_k) {
        return Err(VegetationError::Domain("liquid saturation temperature"));
    }
    let celsius = temperature_k - 273.15;
    let hpa = ESAT_HPA
        .iter()
        .rev()
        .fold(0.0, |sum, coefficient| sum * celsius + coefficient);
    Ok(hpa * 100.0)
}

pub fn saturation_specific_humidity(
    temperature_k: f64,
    pressure_pa: f64,
) -> Result<f64, VegetationError> {
    let esat = saturation_vapor_pressure_pa(temperature_k)?;
    if !pressure_pa.is_finite() || pressure_pa <= 0.378 * esat {
        return Err(VegetationError::Domain("saturation specific humidity"));
    }
    Ok(0.622 * esat / (pressure_pa - 0.378 * esat))
}

pub fn energy_residual(
    absorbed: f64,
    sensible: f64,
    evaporation_kg_m2_s: f64,
    storage_or_conduction: f64,
) -> Result<f64, VegetationError> {
    if [
        absorbed,
        sensible,
        evaporation_kg_m2_s,
        storage_or_conduction,
    ]
    .iter()
    .any(|value| !value.is_finite())
    {
        return Err(VegetationError::Domain("energy ledger"));
    }
    Ok(
        absorbed
            - sensible
            - evaporation_kg_m2_s * LATENT_HEAT_VAPORIZATION
            - storage_or_conduction,
    )
}

#[derive(Clone, Copy, Debug)]
pub struct LeafClassEnergyInput {
    pub lai_total: f64,
    pub lai_dry: f64,
    pub absorbed_shortwave_w_m2: f64,
    pub absorbed_par_w_m2_leaf: f64,
    pub gb_m_s: f64,
    pub emissivity: f64,
    pub biochemical: FvcbInput,
    pub ha_vcmax: f64,
    pub hd_vcmax: f64,
    pub entropy_vcmax: f64,
    pub ha_jmax: f64,
    pub hd_jmax: f64,
    pub entropy_jmax: f64,
    pub ha_kc: f64,
    pub ha_ko: f64,
    pub ha_gamma: f64,
}

#[derive(Clone, Debug)]
pub struct CanopyEnergyInput {
    pub sun: LeafClassEnergyInput,
    pub shade: LeafClassEnergyInput,
    pub wet_leaf_area: f64,
    pub wet_stem_area: f64,
    pub dry_stem_area: f64,
    pub wet_shortwave_w_m2: f64,
    pub dry_stem_shortwave_w_m2: f64,
    pub gb_wet_m_s: f64,
    pub gb_stem_m_s: f64,
    pub wet_emissivity: f64,
    pub stem_emissivity: f64,
    pub longwave_down_w_m2: f64,
    pub longwave_up_w_m2: f64,
    pub air_temperature_k: f64,
    pub qair: f64,
    pub pressure_pa: f64,
    pub atmospheric_co2_pa: f64,
    pub rah_s_m: f64,
    pub raw_s_m: f64,
    pub g0: f64,
    pub g1: f64,
    pub beta_hyd: f64,
    pub liquid_store_kg_m2: f64,
    pub condensation_capacity_kg_m2: f64,
    pub dt_s: f64,
    pub warm_start: [f64; 6],
}

#[derive(Clone, Debug, PartialEq)]
pub struct CanopyEnergySolution {
    pub sun_temperature_k: f64,
    pub shade_temperature_k: f64,
    pub wet_temperature_k: f64,
    pub stem_temperature_k: f64,
    pub canopy_temperature_k: f64,
    pub canopy_specific_humidity: f64,
    pub sun_ci: CiSolution,
    pub shade_ci: CiSolution,
    pub sun_transpiration_kg_m2_s: f64,
    pub shade_transpiration_kg_m2_s: f64,
    pub wet_vapor_kg_m2_s: f64,
    pub wet_store_cap_active: bool,
    pub residuals: Vec<f64>,
    pub diagnostics: SolverDiagnostics,
}

pub fn solve_canopy_energy(
    input: &CanopyEnergyInput,
) -> Result<CanopyEnergySolution, VegetationError> {
    validate_canopy(input)?;
    let atols = [1e-6, 1e-6, 1e-6, 1e-6, 1e-6, 1e-10];
    let energy_scale = (input.sun.absorbed_shortwave_w_m2
        + input.shade.absorbed_shortwave_w_m2
        + input.wet_shortwave_w_m2
        + input.dry_stem_shortwave_w_m2
        + input.longwave_down_w_m2
        + input.longwave_up_w_m2)
        .abs()
        .max(1.0);
    let physical_scales = [
        energy_scale,
        energy_scale,
        energy_scale,
        energy_scale,
        input.air_temperature_k.abs().max(1.0),
        input.qair.abs().max(1e-6),
    ];
    let (state, diagnostics) = damped_newton(
        NewtonSystem::Energy,
        &input.warm_start,
        &[1.0, 1.0, 1.0, 1.0, 1.0, 0.01],
        &atols,
        &physical_scales,
        1e-10,
        1e-8,
        50,
        20,
        |state| canopy_residual(input, state).map(|value| value.0),
    )?;
    let (residuals, sun_ci, shade_ci, sun_e, shade_e, wet, cap) = canopy_residual(input, &state)?;
    Ok(CanopyEnergySolution {
        sun_temperature_k: state[0],
        shade_temperature_k: state[1],
        wet_temperature_k: state[2],
        stem_temperature_k: state[3],
        canopy_temperature_k: state[4],
        canopy_specific_humidity: state[5],
        sun_ci,
        shade_ci,
        sun_transpiration_kg_m2_s: sun_e,
        shade_transpiration_kg_m2_s: shade_e,
        wet_vapor_kg_m2_s: wet,
        wet_store_cap_active: cap,
        residuals,
        diagnostics,
    })
}

type CanopyResidualDetail = (Vec<f64>, CiSolution, CiSolution, f64, f64, f64, bool);
fn canopy_residual(
    input: &CanopyEnergyInput,
    state: &[f64],
) -> Result<CanopyResidualDetail, VegetationError> {
    if state.len() != 6
        || state.iter().any(|v| !v.is_finite())
        || state[..5].iter().any(|t| !(273.15..=373.15).contains(t))
        || state[5] <= 0.0
    {
        return Err(VegetationError::Domain("canopy energy state"));
    }
    let tcan = state[4];
    let qcan = state[5];
    let rho = input.pressure_pa / (287.05 * tcan);
    let sun = leaf_state(input, &input.sun, state[0], tcan, qcan, rho)?;
    let shade = leaf_state(input, &input.shade, state[1], tcan, qcan, rho)?;
    let wet_area = input.wet_leaf_area + input.wet_stem_area;
    let (wet_flux, wet_residual, cap_active) = if wet_area == 0.0 {
        (0.0, state[2] - tcan, false)
    } else {
        let potential = rho
            * input.gb_wet_m_s
            * (saturation_specific_humidity(state[2], input.pressure_pa)? - qcan)
            * wet_area;
        let limit = input.liquid_store_kg_m2 / input.dt_s;
        let condensation_limit = input.condensation_capacity_kg_m2 / input.dt_s;
        if potential < 0.0 && condensation_limit == 0.0 {
            return Err(VegetationError::Unsupported(
                "WET_SURFACE_CONDENSATION_WITHOUT_STORE_CAPACITY",
            ));
        }
        let actual = potential.clamp(-condensation_limit, limit);
        let lw = input.wet_emissivity
            * wet_area
            * (input.longwave_down_w_m2 + input.longwave_up_w_m2
                - 2.0 * STEFAN_BOLTZMANN * state[2].powi(4));
        let sensible = rho * 1_004.64 * input.gb_wet_m_s * (state[2] - tcan) * wet_area;
        (
            actual,
            input.wet_shortwave_w_m2 + lw - sensible - LATENT_HEAT_VAPORIZATION * actual,
            potential > limit || potential < -condensation_limit,
        )
    };
    let stem_residual = if input.dry_stem_area == 0.0 {
        state[3] - tcan
    } else {
        let lw = input.stem_emissivity
            * input.dry_stem_area
            * (input.longwave_down_w_m2 + input.longwave_up_w_m2
                - 2.0 * STEFAN_BOLTZMANN * state[3].powi(4));
        let sensible = rho * 1_004.64 * input.gb_stem_m_s * (state[3] - tcan) * input.dry_stem_area;
        input.dry_stem_shortwave_w_m2 + lw - sensible
    };
    let heat_node = (tcan - input.air_temperature_k) / input.rah_s_m
        - (input.sun.gb_m_s * input.sun.lai_dry * (state[0] - tcan)
            + input.shade.gb_m_s * input.shade.lai_dry * (state[1] - tcan)
            + input.gb_wet_m_s * wet_area * (state[2] - tcan)
            + input.gb_stem_m_s * input.dry_stem_area * (state[3] - tcan));
    let vapor_node = (qcan - input.qair) / input.raw_s_m - (sun.2 + shade.2 + wet_flux) / rho;
    Ok((
        vec![
            sun.1,
            shade.1,
            wet_residual,
            stem_residual,
            heat_node,
            vapor_node,
        ],
        sun.0,
        shade.0,
        sun.2,
        shade.2,
        wet_flux,
        cap_active,
    ))
}

fn leaf_state(
    input: &CanopyEnergyInput,
    class: &LeafClassEnergyInput,
    temperature: f64,
    tcan: f64,
    qcan: f64,
    rho: f64,
) -> Result<(CiSolution, f64, f64), VegetationError> {
    if class.lai_total == 0.0 {
        let ci = CiSolution {
            ci_pa: input.atmospheric_co2_pa,
            cs_pa: input.atmospheric_co2_pa,
            gs_umol_h2o_m2_s: 0.0,
            rs_s_m: 0.0,
            fvcb: crate::photosynthesis::FvcbResult {
                ac: 0.0,
                aj: 0.0,
                ap: 0.0,
                ag: 0.0,
                an: 0.0,
                j: 0.0,
            },
            diagnostics: SolverDiagnostics {
                iterations: 0,
                evaluations: 0,
                backtracks: 0,
                residual_norm: 0.0,
                step_norm: 0.0,
                pivot_failure: false,
            },
        };
        return Ok((ci, temperature - tcan, 0.0));
    }
    let qsat = saturation_specific_humidity(temperature, input.pressure_pa)?;
    let es = qsat * input.pressure_pa / (0.622 + 0.378 * qsat);
    let ecan = qcan * input.pressure_pa / (0.622 + 0.378 * qcan);
    let vpd = (es - ecan) / 1000.0;
    if vpd <= 0.0 {
        return Err(VegetationError::Domain("solved surface VPD"));
    }
    let rb = 1.0 / class.gb_m_s;
    let mut biochemical = class.biochemical;
    biochemical.par_abs = class.absorbed_par_w_m2_leaf;
    let vc_response = peaked_response(
        temperature,
        class.ha_vcmax,
        class.hd_vcmax,
        class.entropy_vcmax,
    )?;
    let j_response = peaked_response(
        temperature,
        class.ha_jmax,
        class.hd_jmax,
        class.entropy_jmax,
    )?;
    biochemical.vcmax *= vc_response;
    biochemical.jmax *= j_response;
    biochemical.tp *= vc_response;
    biochemical.kc_pa *= arrhenius(temperature, class.ha_kc)?;
    biochemical.ko_pa *= arrhenius(temperature, class.ha_ko)?;
    biochemical.gamma_pa *= arrhenius(temperature, class.ha_gamma)?;
    let ci = solve_ci(
        biochemical,
        input.atmospheric_co2_pa,
        rb,
        temperature,
        vpd,
        input.g0,
        input.g1,
        input.pressure_pa,
        input.beta_hyd,
    )?;
    let flux = rho * (qsat - qcan) / (rb + ci.rs_s_m) * class.lai_dry;
    let lw = class.emissivity
        * class.lai_dry
        * (input.longwave_down_w_m2 + input.longwave_up_w_m2
            - 2.0 * STEFAN_BOLTZMANN * temperature.powi(4));
    let sensible = rho * 1_004.64 * class.gb_m_s * (temperature - tcan) * class.lai_dry;
    Ok((
        ci,
        class.absorbed_shortwave_w_m2 + lw - sensible - LATENT_HEAT_VAPORIZATION * flux,
        flux,
    ))
}

fn validate_canopy(input: &CanopyEnergyInput) -> Result<(), VegetationError> {
    let values = [
        input.air_temperature_k,
        input.qair,
        input.pressure_pa,
        input.atmospheric_co2_pa,
        input.rah_s_m,
        input.raw_s_m,
        input.g0,
        input.g1,
        input.beta_hyd,
        input.liquid_store_kg_m2,
        input.condensation_capacity_kg_m2,
        input.dt_s,
        input.sun.gb_m_s,
        input.shade.gb_m_s,
        input.gb_wet_m_s,
        input.gb_stem_m_s,
    ];
    if values.iter().any(|v| !v.is_finite())
        || input.air_temperature_k <= 0.0
        || input.qair <= 0.0
        || input.pressure_pa <= 0.0
        || input.atmospheric_co2_pa <= 0.0
        || input.rah_s_m <= 0.0
        || input.raw_s_m <= 0.0
        || input.g0 < 0.0
        || input.g1 < 0.0
        || !(0.0..=1.0).contains(&input.beta_hyd)
        || input.liquid_store_kg_m2 < 0.0
        || input.condensation_capacity_kg_m2 < 0.0
        || input.dt_s <= 0.0
        || input.sun.gb_m_s <= 0.0
        || input.shade.gb_m_s <= 0.0
        || input.gb_wet_m_s <= 0.0
        || input.gb_stem_m_s <= 0.0
    {
        return Err(VegetationError::Domain("canopy energy input"));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn zero_leaf() -> LeafClassEnergyInput {
        LeafClassEnergyInput {
            lai_total: 0.0,
            lai_dry: 0.0,
            absorbed_shortwave_w_m2: 0.0,
            absorbed_par_w_m2_leaf: 0.0,
            gb_m_s: 0.1,
            emissivity: 0.96,
            biochemical: FvcbInput {
                ci_pa: 30.0,
                oi_pa: 20_265.0,
                gamma_pa: 4.0,
                kc_pa: 40.0,
                ko_pa: 27_000.0,
                vcmax: 0.0,
                jmax: 0.0,
                tp: 0.0,
                rd: 0.0,
                par_abs: 0.0,
            },
            ha_vcmax: 1.0,
            hd_vcmax: 2.0,
            entropy_vcmax: 1.0,
            ha_jmax: 1.0,
            hd_jmax: 2.0,
            entropy_jmax: 1.0,
            ha_kc: 1.0,
            ha_ko: 1.0,
            ha_gamma: 1.0,
        }
    }

    #[test]
    fn condensation_requires_finite_store_capacity() {
        let leaf = zero_leaf();
        let input = CanopyEnergyInput {
            sun: leaf,
            shade: leaf,
            wet_leaf_area: 1.0,
            wet_stem_area: 0.0,
            dry_stem_area: 0.0,
            wet_shortwave_w_m2: 0.0,
            dry_stem_shortwave_w_m2: 0.0,
            gb_wet_m_s: 0.1,
            gb_stem_m_s: 0.1,
            wet_emissivity: 0.96,
            stem_emissivity: 0.96,
            longwave_down_w_m2: 300.0,
            longwave_up_w_m2: 300.0,
            air_temperature_k: 290.0,
            qair: 0.02,
            pressure_pa: 101_325.0,
            atmospheric_co2_pa: 40.0,
            rah_s_m: 100.0,
            raw_s_m: 100.0,
            g0: 25.0,
            g1: 3.5,
            beta_hyd: 1.0,
            liquid_store_kg_m2: 0.0,
            condensation_capacity_kg_m2: 0.0,
            dt_s: 1_800.0,
            warm_start: [290.0, 290.0, 290.0, 290.0, 290.0, 0.02],
        };
        assert_eq!(
            canopy_residual(&input, &input.warm_start),
            Err(VegetationError::Unsupported(
                "WET_SURFACE_CONDENSATION_WITHOUT_STORE_CAPACITY"
            ))
        );
    }

    #[test]
    fn v3_surface_wind_is_friction_velocity_not_reference_wind() {
        let u_star =
            canopy_surface_friction_velocity(2.4, 30.0, 12.0, 1.0).expect("supported neutral wind");
        let expected = 0.4 * 2.4 / ((30.0_f64 - 12.0) / 1.0).ln();
        assert!((u_star - expected).abs() < 1.0e-15);
        assert_ne!(u_star.to_bits(), 2.4_f64.to_bits());
        assert!(canopy_surface_friction_velocity(0.0, 30.0, 12.0, 1.0).is_err());
        assert!(canopy_surface_friction_velocity(2.4, 13.0, 12.0, 1.0).is_err());
    }
}
