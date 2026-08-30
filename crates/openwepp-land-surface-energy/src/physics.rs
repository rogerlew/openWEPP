//! Exact constitutive primitives for `OPENWEPP_SNOW_FREE_LSE_V1`.
//!
//! These functions are intentionally free of owner mutation and numerical
//! acceptance decisions.  They transcribe the equations admitted by
//! `SC-LANDSURFACEENERGY-001@3`; the joint solver recomputes them at every
//! nonlinear residual evaluation.

// Equation-symbol continuity and compact public authority primitives make
// these names intentionally similar; each public Result fails on the domain
// documented by its typed operands.
#![allow(
    clippy::items_after_statements,
    clippy::missing_errors_doc,
    clippy::similar_names
)]

use std::f64::consts::PI;

use crate::LandSurfaceEnergyError;

pub const STEFAN_BOLTZMANN_W_M2_K4: f64 = 5.670_374_419e-8;
pub const WATER_HEAT_CAPACITY_J_KG_K: f64 = 4_218.0;
pub const LITTER_ICE_HEAT_CAPACITY_J_KG_K: f64 = 2_106.0;
pub const LITTER_ICE_DENSITY_KG_M3: f64 = 920.0;
pub const WATER_DENSITY_KG_M3: f64 = 1_000.0;
pub const LITTER_FUSION_ENTHALPY_J_KG: f64 = 333_700.0;
pub const LITTER_ICE_TIMESCALE_S: f64 = 3_300.0;
pub const LITTER_ICE_VOLUMETRIC_CAPACITY: f64 = 0.85;
pub const AIR_HEAT_CAPACITY_J_KG_K: f64 = 1_004.64;
pub const DRY_AIR_GAS_CONSTANT_J_KG_K: f64 = 287.05;
pub const WATER_VAPOR_GAS_CONSTANT_J_KG_K: f64 = 461.5;
pub const REFERENCE_TEMPERATURE_K: f64 = 273.15;
pub const VON_KARMAN: f64 = 0.4;
pub const ENERGY_ABSOLUTE_TOLERANCE_W_M2: f64 = 1.0e-6;
pub const ENERGY_RELATIVE_TOLERANCE: f64 = 1.0e-10;
pub const WATER_ABSOLUTE_TOLERANCE_KG_M2_S: f64 = 1.0e-12;
pub const WATER_RELATIVE_TOLERANCE: f64 = 1.0e-9;

fn finite(value: f64, field: &'static str) -> Result<f64, LandSurfaceEnergyError> {
    if value.is_finite() {
        Ok(value)
    } else {
        Err(LandSurfaceEnergyError::NonFinite(field))
    }
}

fn positive(value: f64, field: &'static str) -> Result<f64, LandSurfaceEnergyError> {
    finite(value, field)?;
    if value > 0.0 {
        Ok(value)
    } else {
        Err(LandSurfaceEnergyError::ConstitutiveDomain(field))
    }
}

/// Saturation specific humidity used by the checksum-bound authority oracle.
pub fn saturation_specific_humidity(
    temperature_k: f64,
    pressure_pa: f64,
) -> Result<f64, LandSurfaceEnergyError> {
    positive(temperature_k, "surface_temperature_k")?;
    positive(pressure_pa, "air_pressure_pa")?;
    let temperature_c = temperature_k - REFERENCE_TEMPERATURE_K;
    let denominator = temperature_c + 243.5;
    if denominator <= 0.0 {
        return Err(LandSurfaceEnergyError::ConstitutiveDomain(
            "saturation_vapor_pressure_temperature",
        ));
    }
    let vapor_pressure_pa = 611.2 * (17.67 * temperature_c / denominator).exp();
    let mixing_denominator = pressure_pa - 0.378 * vapor_pressure_pa;
    if !vapor_pressure_pa.is_finite() || mixing_denominator <= 0.0 {
        return Err(LandSurfaceEnergyError::ConstitutiveDomain(
            "saturation_specific_humidity",
        ));
    }
    finite(
        0.622 * vapor_pressure_pa / mixing_denominator,
        "saturation_specific_humidity",
    )
}

#[must_use]
pub fn liquid_enthalpy_j_kg(temperature_k: f64) -> f64 {
    WATER_HEAT_CAPACITY_J_KG_K * (temperature_k - REFERENCE_TEMPERATURE_K)
}

#[must_use]
pub fn vaporization_enthalpy_j_kg(temperature_k: f64) -> f64 {
    2_501_000.0 - 2_369.0 * (temperature_k - REFERENCE_TEMPERATURE_K)
}

/// Sublimation enthalpy selected by the V3 litter authority. The fusion term
/// is kept explicit so an ice-vapor operand cannot alias liquid vapor.
#[must_use]
pub fn sublimation_enthalpy_j_kg(temperature_k: f64) -> f64 {
    vaporization_enthalpy_j_kg(temperature_k) + LITTER_FUSION_ENTHALPY_J_KG
}

pub fn vapor_export_w_m2(
    signed_vapor_kg_m2_s: f64,
    surface_temperature_k: f64,
) -> Result<f64, LandSurfaceEnergyError> {
    finite(signed_vapor_kg_m2_s, "signed_vapor_kg_m2_s")?;
    positive(surface_temperature_k, "surface_temperature_k")?;
    finite(
        signed_vapor_kg_m2_s
            * (liquid_enthalpy_j_kg(surface_temperature_k)
                + vaporization_enthalpy_j_kg(surface_temperature_k)),
        "vapor_export_w_m2",
    )
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct OpenNeutralGeometry {
    pub reference_height_m: f64,
    pub roughness_momentum_m: f64,
    pub roughness_heat_m: f64,
    pub roughness_vapor_m: f64,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct NeutralResistances {
    pub heat_s_m: f64,
    pub vapor_s_m: f64,
}

pub fn open_neutral_resistances(
    geometry: OpenNeutralGeometry,
    reference_wind_m_s: f64,
) -> Result<NeutralResistances, LandSurfaceEnergyError> {
    positive(reference_wind_m_s, "reference_wind_m_s")?;
    let values = [
        geometry.reference_height_m,
        geometry.roughness_momentum_m,
        geometry.roughness_heat_m,
        geometry.roughness_vapor_m,
    ];
    for value in values {
        positive(value, "open_neutral_geometry")?;
    }
    let largest_roughness = geometry
        .roughness_momentum_m
        .max(geometry.roughness_heat_m)
        .max(geometry.roughness_vapor_m);
    if geometry.reference_height_m <= largest_roughness {
        return Err(LandSurfaceEnergyError::UnsupportedDomain(
            "open_neutral_geometry",
        ));
    }
    let momentum = (geometry.reference_height_m / geometry.roughness_momentum_m).ln();
    let common = momentum / (VON_KARMAN * VON_KARMAN * reference_wind_m_s);
    let heat = common * (geometry.reference_height_m / geometry.roughness_heat_m).ln();
    let vapor = common * (geometry.reference_height_m / geometry.roughness_vapor_m).ln();
    positive(heat, "open_heat_resistance_s_m")?;
    positive(vapor, "open_vapor_resistance_s_m")?;
    Ok(NeutralResistances {
        heat_s_m: heat,
        vapor_s_m: vapor,
    })
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct UnderCanopyGeometry {
    pub canopy_height_m: f64,
    pub canopy_roughness_m: f64,
    pub reference_height_m: f64,
    pub leaf_area_index: f64,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct UnderCanopyResistance {
    pub reynolds_number: f64,
    pub drag_coefficient: f64,
    pub displacement_m: f64,
    pub friction_velocity_m_s: f64,
    pub eddy_diffusivity_m2_s: f64,
    pub resistance_s_m: f64,
}

/// ISBA-MEB 54--63 under the V1 neutral constants.
pub fn under_canopy_neutral_resistance(
    geometry: UnderCanopyGeometry,
    reference_wind_m_s: f64,
) -> Result<UnderCanopyResistance, LandSurfaceEnergyError> {
    for value in [
        geometry.canopy_height_m,
        geometry.canopy_roughness_m,
        geometry.reference_height_m,
        geometry.leaf_area_index,
        reference_wind_m_s,
    ] {
        positive(value, "under_canopy_geometry")?;
    }
    const PHI_V: f64 = 2.0;
    const Z0_G: f64 = 0.007;
    const CHI_L: f64 = 0.12;
    const U_L: f64 = 1.0;
    const L_W: f64 = 0.02;
    const NU: f64 = 1.5e-5;
    let reynolds = U_L * L_W / NU;
    let drag = 1.328 * (2.0 / reynolds.sqrt()) + 0.45 * ((1.0 - CHI_L) / PI).powf(1.6);
    let displacement =
        1.1 * geometry.canopy_height_m * (1.0 + (drag * geometry.leaf_area_index).powf(0.25)).ln();
    if !(geometry.canopy_height_m > displacement + geometry.canopy_roughness_m
        && displacement + geometry.canopy_roughness_m > Z0_G
        && geometry.reference_height_m - displacement >= geometry.canopy_height_m - displacement)
    {
        return Err(LandSurfaceEnergyError::UnsupportedDomain(
            "under_canopy_geometry",
        ));
    }
    let friction_velocity = VON_KARMAN * reference_wind_m_s
        / ((geometry.canopy_height_m - displacement) / geometry.canopy_roughness_m).ln();
    let eddy = VON_KARMAN * friction_velocity * (geometry.canopy_height_m - displacement);
    let resistance = geometry.canopy_height_m / (PHI_V * eddy)
        * ((PHI_V * (1.0 - Z0_G / geometry.canopy_height_m)).exp()
            - (PHI_V
                * (1.0 - (displacement + geometry.canopy_roughness_m) / geometry.canopy_height_m))
                .exp());
    for value in [drag, friction_velocity, eddy, resistance] {
        positive(value, "under_canopy_resistance")?;
    }
    Ok(UnderCanopyResistance {
        reynolds_number: reynolds,
        drag_coefficient: drag,
        displacement_m: displacement,
        friction_velocity_m_s: friction_velocity,
        eddy_diffusivity_m2_s: eddy,
        resistance_s_m: resistance,
    })
}

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct BandDirectionalFluxes {
    pub direct_vis: f64,
    pub diffuse_vis: f64,
    pub direct_nir: f64,
    pub diffuse_nir: f64,
}

impl BandDirectionalFluxes {
    pub fn validate_nonnegative(self) -> Result<(), LandSurfaceEnergyError> {
        for value in [
            self.direct_vis,
            self.diffuse_vis,
            self.direct_nir,
            self.diffuse_nir,
        ] {
            finite(value, "band_directional_flux")?;
            if value < 0.0 {
                return Err(LandSurfaceEnergyError::ConstitutiveDomain(
                    "band_directional_flux",
                ));
            }
        }
        Ok(())
    }

    #[must_use]
    pub fn total(self) -> f64 {
        self.direct_vis + self.diffuse_vis + self.direct_nir + self.diffuse_nir
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ShortwavePartition {
    pub absorbed: BandDirectionalFluxes,
    pub reflected: BandDirectionalFluxes,
}

pub fn partition_ground_shortwave(
    terminal: BandDirectionalFluxes,
    vis_albedo: f64,
    nir_albedo: f64,
) -> Result<ShortwavePartition, LandSurfaceEnergyError> {
    terminal.validate_nonnegative()?;
    for albedo in [vis_albedo, nir_albedo] {
        finite(albedo, "ground_albedo")?;
        if !(0.0..=1.0).contains(&albedo) {
            return Err(LandSurfaceEnergyError::ConstitutiveDomain("ground_albedo"));
        }
    }
    let absorbed = BandDirectionalFluxes {
        direct_vis: terminal.direct_vis * (1.0 - vis_albedo),
        diffuse_vis: terminal.diffuse_vis * (1.0 - vis_albedo),
        direct_nir: terminal.direct_nir * (1.0 - nir_albedo),
        diffuse_nir: terminal.diffuse_nir * (1.0 - nir_albedo),
    };
    Ok(ShortwavePartition {
        reflected: BandDirectionalFluxes {
            direct_vis: terminal.direct_vis - absorbed.direct_vis,
            diffuse_vis: terminal.diffuse_vis - absorbed.diffuse_vis,
            direct_nir: terminal.direct_nir - absorbed.direct_nir,
            diffuse_nir: terminal.diffuse_nir - absorbed.diffuse_nir,
        },
        absorbed,
    })
}

#[derive(Clone, Debug, PartialEq)]
pub struct CanopyLongwaveLayer {
    pub clumping_index: f64,
    pub leaf_area_index: f64,
    pub stem_area_index: f64,
    /// Ordered sun-leaf, shade-leaf, wet-surface, dry-stem emissive areas.
    pub component_areas: [f64; 4],
    /// Ordered temperatures matching `component_areas`.
    pub component_temperatures_k: [f64; 4],
}

#[derive(Clone, Debug, PartialEq)]
pub struct CanopyLongwaveResult {
    pub transmissivities: Vec<f64>,
    pub downward_boundaries_w_m2: Vec<f64>,
    pub upward_boundaries_w_m2: Vec<f64>,
    pub component_net_w_m2: Vec<[f64; 4]>,
    pub ground_net_w_m2: f64,
    pub top_upward_w_m2: f64,
}

pub fn reciprocal_longwave_column(
    atmospheric_downward_w_m2: f64,
    ground_temperature_k: f64,
    layers: &[CanopyLongwaveLayer],
) -> Result<CanopyLongwaveResult, LandSurfaceEnergyError> {
    finite(
        atmospheric_downward_w_m2,
        "atmospheric_downward_longwave_w_m2",
    )?;
    if atmospheric_downward_w_m2 < 0.0 {
        return Err(LandSurfaceEnergyError::ConstitutiveDomain(
            "atmospheric_downward_longwave_w_m2",
        ));
    }
    positive(ground_temperature_k, "ground_temperature_k")?;
    let mut tau = Vec::with_capacity(layers.len());
    let mut emission = Vec::with_capacity(layers.len());
    let mut weights = Vec::with_capacity(layers.len());
    for layer in layers {
        finite(layer.clumping_index, "clumping_index")?;
        if !(0.0..=1.0).contains(&layer.clumping_index) || layer.clumping_index == 0.0 {
            return Err(LandSurfaceEnergyError::ConstitutiveDomain("clumping_index"));
        }
        for value in [layer.leaf_area_index, layer.stem_area_index] {
            finite(value, "plant_area")?;
            if value < 0.0 {
                return Err(LandSurfaceEnergyError::ConstitutiveDomain("plant_area"));
            }
        }
        let area_sum: f64 = layer.component_areas.iter().sum();
        for (&area, &temperature) in layer
            .component_areas
            .iter()
            .zip(layer.component_temperatures_k.iter())
        {
            finite(area, "component_emissive_area")?;
            if area < 0.0 {
                return Err(LandSurfaceEnergyError::ConstitutiveDomain(
                    "component_emissive_area",
                ));
            }
            positive(temperature, "component_temperature_k")?;
        }
        if area_sum == 0.0 {
            tau.push(1.0);
            emission.push(0.0);
            weights.push([0.0; 4]);
        } else {
            let layer_tau =
                (-0.8 * layer.clumping_index * (layer.leaf_area_index + layer.stem_area_index))
                    .exp();
            let layer_weights = layer.component_areas.map(|area| area / area_sum);
            let layer_emission = layer_weights
                .iter()
                .zip(layer.component_temperatures_k.iter())
                .map(|(weight, temperature)| {
                    weight * STEFAN_BOLTZMANN_W_M2_K4 * temperature.powi(4)
                })
                .sum();
            tau.push(layer_tau);
            emission.push(layer_emission);
            weights.push(layer_weights);
        }
    }
    let mut downward = Vec::with_capacity(layers.len() + 1);
    downward.push(atmospheric_downward_w_m2);
    for index in 0..layers.len() {
        downward.push(tau[index] * downward[index] + (1.0 - tau[index]) * emission[index]);
    }
    let ground_up = STEFAN_BOLTZMANN_W_M2_K4 * ground_temperature_k.powi(4);
    let mut upward = vec![0.0; layers.len() + 1];
    upward[layers.len()] = ground_up;
    for index in (0..layers.len()).rev() {
        upward[index] = tau[index] * upward[index + 1] + (1.0 - tau[index]) * emission[index];
    }
    let mut component_net = Vec::with_capacity(layers.len());
    for (index, layer) in layers.iter().enumerate() {
        let mut values = [0.0; 4];
        for component in 0..4 {
            values[component] = weights[index][component]
                * (1.0 - tau[index])
                * (downward[index] + upward[index + 1])
                - 2.0
                    * weights[index][component]
                    * (1.0 - tau[index])
                    * STEFAN_BOLTZMANN_W_M2_K4
                    * layer.component_temperatures_k[component].powi(4);
        }
        component_net.push(values);
    }
    Ok(CanopyLongwaveResult {
        transmissivities: tau,
        ground_net_w_m2: downward[layers.len()] - ground_up,
        top_upward_w_m2: upward[0],
        downward_boundaries_w_m2: downward,
        upward_boundaries_w_m2: upward,
        component_net_w_m2: component_net,
    })
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct BareSoilVaporOperands {
    pub top_layer_liquid_kg_m2: f64,
    pub top_layer_ice_kg_m2: f64,
    pub top_layer_depth_m: f64,
    pub porosity: f64,
    pub saturated_matric_potential_mm: f64,
    pub clapp_hornberger_b: f64,
    pub theta_initial: f64,
    pub surface_temperature_k: f64,
    pub recipient_specific_humidity_kg_kg: f64,
    pub pressure_pa: f64,
    pub aerodynamic_vapor_resistance_s_m: f64,
    pub moist_air_density_kg_m3: f64,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct BareSoilVaporResult {
    pub signed_flux_kg_m2_s: f64,
    pub saturation: f64,
    pub volumetric_liquid: f64,
    pub matric_potential_mm: f64,
    pub kelvin_factor: f64,
    pub theta_air: f64,
    pub dry_layer_m: f64,
    pub pore_tortuosity: f64,
    pub vapor_diffusivity_m2_s: f64,
    pub soil_resistance_s_m: f64,
    pub surface_specific_humidity_kg_kg: f64,
    pub zero_flux_branch: bool,
}

pub fn bare_soil_vapor(
    operands: BareSoilVaporOperands,
) -> Result<BareSoilVaporResult, LandSurfaceEnergyError> {
    for value in [
        operands.top_layer_liquid_kg_m2,
        operands.top_layer_ice_kg_m2,
        operands.recipient_specific_humidity_kg_kg,
    ] {
        finite(value, "bare_soil_nonnegative_operand")?;
        if value < 0.0 {
            return Err(LandSurfaceEnergyError::ConstitutiveDomain(
                "bare_soil_nonnegative_operand",
            ));
        }
    }
    for value in [
        operands.top_layer_depth_m,
        operands.porosity,
        operands.clapp_hornberger_b,
        operands.theta_initial,
        operands.surface_temperature_k,
        operands.pressure_pa,
        operands.aerodynamic_vapor_resistance_s_m,
        operands.moist_air_density_kg_m3,
    ] {
        positive(value, "bare_soil_positive_operand")?;
    }
    finite(
        operands.saturated_matric_potential_mm,
        "saturated_matric_potential_mm",
    )?;
    if operands.saturated_matric_potential_mm >= 0.0 || operands.porosity > 1.0 {
        return Err(LandSurfaceEnergyError::ConstitutiveDomain(
            "bare_soil_soil_parameter",
        ));
    }
    let saturation = ((operands.top_layer_liquid_kg_m2 / 1_000.0
        + operands.top_layer_ice_kg_m2 / 917.0)
        / (operands.top_layer_depth_m * operands.porosity))
        .clamp(0.01, 1.0);
    let theta = operands.top_layer_liquid_kg_m2 / (1_000.0 * operands.top_layer_depth_m);
    let matric = (operands.saturated_matric_potential_mm
        * saturation.powf(-operands.clapp_hornberger_b))
    .max(-1.0e8);
    let kelvin = (matric * 9.806_65
        / (1_000.0 * WATER_VAPOR_GAS_CONSTANT_J_KG_K * operands.surface_temperature_k))
        .exp();
    let theta_air = operands.porosity
        * (operands.saturated_matric_potential_mm / -1.0e7).powf(1.0 / operands.clapp_hornberger_b);
    let dry_denominator = operands.theta_initial - theta_air;
    if dry_denominator <= 0.0 {
        return Err(LandSurfaceEnergyError::ConstitutiveDomain(
            "dry_surface_layer_denominator",
        ));
    }
    let dry_layer = if theta < operands.theta_initial {
        0.015 * (operands.theta_initial - theta) / dry_denominator
    } else {
        0.0
    };
    let pore_air = operands.porosity - theta_air;
    if pore_air <= 0.0 {
        return Err(LandSurfaceEnergyError::ConstitutiveDomain(
            "bare_soil_pore_air",
        ));
    }
    let tortuosity =
        pore_air.powi(2) * (pore_air / operands.porosity).powf(3.0 / operands.clapp_hornberger_b);
    let diffusivity =
        2.12e-5 * (operands.surface_temperature_k / REFERENCE_TEMPERATURE_K).powf(1.75);
    let soil_resistance = dry_layer / (diffusivity * tortuosity);
    let saturated_q =
        saturation_specific_humidity(operands.surface_temperature_k, operands.pressure_pa)?;
    let raw_surface_q = kelvin * saturated_q;
    let zero_flux = saturated_q > operands.recipient_specific_humidity_kg_kg
        && operands.recipient_specific_humidity_kg_kg > raw_surface_q;
    let surface_q = if zero_flux {
        operands.recipient_specific_humidity_kg_kg
    } else {
        raw_surface_q
    };
    let flux = operands.moist_air_density_kg_m3
        * (surface_q - operands.recipient_specific_humidity_kg_kg)
        / (operands.aerodynamic_vapor_resistance_s_m + soil_resistance);
    Ok(BareSoilVaporResult {
        signed_flux_kg_m2_s: finite(flux, "bare_soil_vapor_flux")?,
        saturation,
        volumetric_liquid: theta,
        matric_potential_mm: matric,
        kelvin_factor: kelvin,
        theta_air,
        dry_layer_m: dry_layer,
        pore_tortuosity: tortuosity,
        vapor_diffusivity_m2_s: diffusivity,
        soil_resistance_s_m: soil_resistance,
        surface_specific_humidity_kg_kg: surface_q,
        zero_flux_branch: zero_flux,
    })
}

pub fn litter_relative_humidity(
    liquid_kg_m2: f64,
    capacity_kg_m2: f64,
) -> Result<f64, LandSurfaceEnergyError> {
    finite(liquid_kg_m2, "litter_liquid_kg_m2")?;
    positive(capacity_kg_m2, "litter_capacity_kg_m2")?;
    if liquid_kg_m2 < 0.0 || liquid_kg_m2 > capacity_kg_m2 {
        return Err(LandSurfaceEnergyError::ConstitutiveDomain(
            "litter_liquid_capacity",
        ));
    }
    Ok(0.5 * (1.0 - (PI * liquid_kg_m2 / capacity_kg_m2).cos()))
}

pub fn harmonic_interface_conductance_w_m2_k(
    depth_a_m: f64,
    conductivity_a_w_m_k: f64,
    depth_b_m: f64,
    conductivity_b_w_m_k: f64,
) -> Result<f64, LandSurfaceEnergyError> {
    for value in [
        depth_a_m,
        conductivity_a_w_m_k,
        depth_b_m,
        conductivity_b_w_m_k,
    ] {
        positive(value, "thermal_interface_operand")?;
    }
    Ok(2.0 / (depth_a_m / conductivity_a_w_m_k + depth_b_m / conductivity_b_w_m_k))
}

#[must_use]
pub fn energy_tolerance(component_scale_w_m2: f64) -> f64 {
    ENERGY_ABSOLUTE_TOLERANCE_W_M2 + ENERGY_RELATIVE_TOLERANCE * component_scale_w_m2.max(1.0)
}

#[must_use]
pub fn water_tolerance(component_scale_kg_m2_s: f64) -> f64 {
    WATER_ABSOLUTE_TOLERANCE_KG_M2_S + WATER_RELATIVE_TOLERANCE * component_scale_kg_m2_s.max(0.0)
}

#[cfg(test)]
mod tests {
    #![allow(clippy::float_cmp)] // exact frozen-oracle and poison identities
    use super::*;

    #[test]
    fn open_resistances_match_frozen_four_layer_vector() {
        let result = open_neutral_resistances(
            OpenNeutralGeometry {
                reference_height_m: 20.0,
                roughness_momentum_m: 0.12,
                roughness_heat_m: 0.015,
                roughness_vapor_m: 0.010,
            },
            2.4,
        )
        .expect("valid vector");
        assert!((result.heat_s_m - 95.864_133_696_051_91).abs() < 1.0e-12);
        assert!((result.vapor_s_m - 101.266_107_118_142_66).abs() < 1.0e-12);
    }

    #[test]
    fn shortwave_preserves_every_band_and_direction() {
        let terminal = BandDirectionalFluxes {
            direct_vis: 91.0,
            diffuse_vis: 31.0,
            direct_nir: 117.0,
            diffuse_nir: 39.0,
        };
        let result = partition_ground_shortwave(terminal, 0.18, 0.31).expect("valid");
        assert_eq!(result.absorbed.direct_vis, 74.62);
        assert_eq!(result.absorbed.diffuse_vis, 25.42);
        assert_eq!(result.absorbed.direct_nir, 80.729_999_999_999_99);
        assert_eq!(result.absorbed.diffuse_nir, 26.909_999_999_999_997);
        assert!(
            (terminal.total() - result.absorbed.total() - result.reflected.total()).abs() < 1e-12
        );
    }

    #[test]
    fn signed_vapor_enthalpy_is_not_latent_only_or_unsigned() {
        let evaporation = vapor_export_w_m2(2.0e-5, 293.15).expect("valid");
        let condensation = vapor_export_w_m2(-2.0e-5, 293.15).expect("valid");
        assert!(evaporation > 0.0);
        assert_eq!(condensation, -evaporation);
        assert_ne!(evaporation, 2.0e-5 * vaporization_enthalpy_j_kg(293.15));
    }

    #[test]
    fn reciprocal_longwave_keeps_component_temperature_identity() {
        let mut layer = CanopyLongwaveLayer {
            clumping_index: 0.82,
            leaf_area_index: 3.0,
            stem_area_index: 0.7,
            component_areas: [1.2, 1.8, 0.4, 0.3],
            component_temperatures_k: [299.0, 295.0, 294.0, 296.0],
        };
        let first = reciprocal_longwave_column(350.0, 291.0, &[layer.clone()]).expect("valid");
        layer.component_temperatures_k.swap(0, 1);
        let swapped = reciprocal_longwave_column(350.0, 291.0, &[layer]).expect("valid");
        assert_ne!(first.component_net_w_m2[0], swapped.component_net_w_m2[0]);
        let first_total: f64 = first.component_net_w_m2[0].iter().sum();
        let closure = 350.0 - first.top_upward_w_m2 - first.ground_net_w_m2 - first_total;
        assert!(closure.abs() < 1.0e-10);
    }

    #[test]
    fn wind_floor_is_not_admitted() {
        let error = open_neutral_resistances(
            OpenNeutralGeometry {
                reference_height_m: 20.0,
                roughness_momentum_m: 0.12,
                roughness_heat_m: 0.015,
                roughness_vapor_m: 0.010,
            },
            0.0,
        )
        .expect_err("calm is unsupported");
        assert_eq!(
            error,
            LandSurfaceEnergyError::ConstitutiveDomain("reference_wind_m_s")
        );
    }
}
