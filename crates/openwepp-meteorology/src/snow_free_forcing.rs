//! Contract-bound atmospheric primitives for `SC-SNOWFREEFORCING-001`.

use thiserror::Error;

const SIGMA_W_M2_K4: f64 = 5.670_374_419e-8;
const STANDARD_PRESSURE_KPA: f64 = 101.325;
pub const LIQUID_HEAT_CAPACITY_J_KG_K: f64 = 4_218.0;

#[must_use]
pub fn celsius_to_kelvin(value_c: f64) -> f64 {
    value_c + 273.15
}

#[must_use]
pub fn kilopascals_to_pascals(value_kpa: f64) -> f64 {
    value_kpa * 1_000.0
}

#[must_use]
pub fn liquid_specific_enthalpy_j_kg(temperature_k: f64) -> f64 {
    LIQUID_HEAT_CAPACITY_J_KG_K * (temperature_k - 273.15)
}

/// Four-way Weiss--Norman shortwave energy partition.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct WeissNormanPartition {
    pub direct_visible_w_m2: f64,
    pub diffuse_visible_w_m2: f64,
    pub direct_nir_w_m2: f64,
    pub diffuse_nir_w_m2: f64,
}

/// Typed snow-free atmospheric-provider failure.
#[derive(Clone, Debug, Error, PartialEq)]
pub enum SnowFreeAtmosphericError {
    #[error("nonfinite atmospheric operand {field}: {value}")]
    NonFinite { field: &'static str, value: f64 },
    #[error("atmospheric operand outside authority {field}: {value}")]
    Domain { field: &'static str, value: f64 },
    #[error("Weiss-Norman component closure failed: expected {expected}, found {found}")]
    ShortwaveClosure { expected: f64, found: f64 },
}

fn finite(field: &'static str, value: f64) -> Result<f64, SnowFreeAtmosphericError> {
    if value.is_finite() {
        Ok(value)
    } else {
        Err(SnowFreeAtmosphericError::NonFinite { field, value })
    }
}

/// FAO-56 station-elevation pressure in kPa.
///
/// # Errors
///
/// Returns a typed failure for nonfinite elevation or a nonpositive pressure
/// domain.
pub fn fao56_station_pressure_kpa(elevation_m: f64) -> Result<f64, SnowFreeAtmosphericError> {
    finite("elevation_m", elevation_m)?;
    let base = (293.0 - 0.0065 * elevation_m) / 293.0;
    if base <= 0.0 {
        return Err(SnowFreeAtmosphericError::Domain {
            field: "fao56_pressure_base",
            value: base,
        });
    }
    let pressure = finite("pressure_kpa", 101.3 * base.powf(5.26))?;
    if pressure <= 0.0 {
        return Err(SnowFreeAtmosphericError::Domain {
            field: "pressure_kpa",
            value: pressure,
        });
    }
    Ok(pressure)
}

/// Dilley--O'Brien clear-sky longwave with Unsworth--Monteith cloud correction.
///
/// # Errors
///
/// Returns a typed failure for nonfinite operands, invalid thermodynamic
/// inputs, or cloud fraction outside `[0, 1]`.
pub fn atmospheric_longwave_dilley_unsworth(
    air_temperature_k: f64,
    actual_vapor_pressure_kpa: f64,
    cloud_fraction: f64,
) -> Result<f64, SnowFreeAtmosphericError> {
    for (field, value) in [
        ("air_temperature_k", air_temperature_k),
        ("actual_vapor_pressure_kpa", actual_vapor_pressure_kpa),
        ("cloud_fraction", cloud_fraction),
    ] {
        finite(field, value)?;
    }
    if air_temperature_k <= 0.0 || actual_vapor_pressure_kpa < 0.0 {
        return Err(SnowFreeAtmosphericError::Domain {
            field: "longwave_thermodynamic_domain",
            value: air_temperature_k.min(actual_vapor_pressure_kpa),
        });
    }
    if !(0.0..=1.0).contains(&cloud_fraction) {
        return Err(SnowFreeAtmosphericError::Domain {
            field: "cloud_fraction",
            value: cloud_fraction,
        });
    }
    let precipitable_water = 4650.0 * actual_vapor_pressure_kpa / air_temperature_k;
    let clear_w_m2 = 59.38
        + 113.7 * (air_temperature_k / 273.16).powf(6.0)
        + 96.96 * (precipitable_water / 25.0).sqrt();
    let blackbody = SIGMA_W_M2_K4 * air_temperature_k.powf(4.0);
    let clear_emissivity = clear_w_m2 / blackbody;
    finite(
        "atmospheric_downward_longwave_w_m2",
        ((1.0 - 0.84 * cloud_fraction) * clear_emissivity + 0.84 * cloud_fraction) * blackbody,
    )
}

/// Complete Weiss--Norman (1985) direct/diffuse visible/NIR partition.
///
/// # Errors
///
/// Returns a typed failure for nonfinite or out-of-domain operands, invalid
/// component potentials, or failure of the four-component energy closure.
pub fn weiss_norman_partition(
    global_horizontal_shortwave_w_m2: f64,
    solar_zenith_cosine: f64,
    pressure_kpa: f64,
) -> Result<WeissNormanPartition, SnowFreeAtmosphericError> {
    for (field, value) in [
        (
            "global_horizontal_shortwave_w_m2",
            global_horizontal_shortwave_w_m2,
        ),
        ("solar_zenith_cosine", solar_zenith_cosine),
        ("pressure_kpa", pressure_kpa),
    ] {
        finite(field, value)?;
    }
    if global_horizontal_shortwave_w_m2 < 0.0 {
        return Err(SnowFreeAtmosphericError::Domain {
            field: "global_horizontal_shortwave_w_m2",
            value: global_horizontal_shortwave_w_m2,
        });
    }
    if global_horizontal_shortwave_w_m2 == 0.0 {
        return Ok(WeissNormanPartition {
            direct_visible_w_m2: 0.0,
            diffuse_visible_w_m2: 0.0,
            direct_nir_w_m2: 0.0,
            diffuse_nir_w_m2: 0.0,
        });
    }
    if solar_zenith_cosine <= 0.0 || pressure_kpa <= 0.0 {
        return Err(SnowFreeAtmosphericError::Domain {
            field: "positive_shortwave_mu_pressure",
            value: solar_zenith_cosine.min(pressure_kpa),
        });
    }
    let air_mass = 1.0 / solar_zenith_cosine;
    let pressure_ratio = pressure_kpa / STANDARD_PRESSURE_KPA;
    let direct_vis_potential =
        600.0 * (-0.185 * pressure_ratio * air_mass).exp() * solar_zenith_cosine;
    let diffuse_vis_potential =
        0.4 * (600.0 - direct_vis_potential / solar_zenith_cosine) * solar_zenith_cosine;
    let log_air_mass = air_mass.log10();
    let water_absorption =
        1320.0 * 10.0_f64.powf(-1.1950 + 0.4459 * log_air_mass - 0.0345 * log_air_mass.powi(2));
    let direct_nir_potential = (720.0 * (-0.06 * pressure_ratio * air_mass).exp()
        - water_absorption)
        * solar_zenith_cosine;
    let diffuse_nir_potential = 0.6
        * (720.0 - direct_nir_potential / solar_zenith_cosine - water_absorption)
        * solar_zenith_cosine;
    let total_vis = direct_vis_potential + diffuse_vis_potential;
    let total_nir = direct_nir_potential + diffuse_nir_potential;
    let total_potential = total_vis + total_nir;
    if total_vis <= 0.0 || total_nir <= 0.0 || total_potential <= 0.0 {
        return Err(SnowFreeAtmosphericError::Domain {
            field: "weiss_norman_potential",
            value: total_vis.min(total_nir).min(total_potential),
        });
    }
    let ratio = global_horizontal_shortwave_w_m2 / total_potential;
    let visible = global_horizontal_shortwave_w_m2 * total_vis / total_potential;
    let nir = global_horizontal_shortwave_w_m2 * total_nir / total_potential;
    let visible_beam_fraction = (direct_vis_potential / total_vis
        * (1.0 - ((0.9 - ratio.min(0.9)) / 0.7).powf(2.0 / 3.0)))
    .max(0.0);
    let nir_beam_fraction = (direct_nir_potential / total_nir
        * (1.0 - ((0.88 - ratio.min(0.88)) / 0.68).powf(2.0 / 3.0)))
    .max(0.0);
    let value = WeissNormanPartition {
        direct_visible_w_m2: visible * visible_beam_fraction,
        diffuse_visible_w_m2: visible * (1.0 - visible_beam_fraction),
        direct_nir_w_m2: nir * nir_beam_fraction,
        diffuse_nir_w_m2: nir * (1.0 - nir_beam_fraction),
    };
    for component in [
        value.direct_visible_w_m2,
        value.diffuse_visible_w_m2,
        value.direct_nir_w_m2,
        value.diffuse_nir_w_m2,
    ] {
        if !component.is_finite() || component < 0.0 {
            return Err(SnowFreeAtmosphericError::Domain {
                field: "weiss_norman_component",
                value: component,
            });
        }
    }
    let found = value.direct_visible_w_m2
        + value.diffuse_visible_w_m2
        + value.direct_nir_w_m2
        + value.diffuse_nir_w_m2;
    let tolerance = 1.0e-12 * global_horizontal_shortwave_w_m2.abs().max(1.0);
    if (found - global_horizontal_shortwave_w_m2).abs() > tolerance {
        return Err(SnowFreeAtmosphericError::ShortwaveClosure {
            expected: global_horizontal_shortwave_w_m2,
            found,
        });
    }
    Ok(value)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn low_transmissivity_is_nonnegative_and_closes() {
        let value = weiss_norman_partition(0.1, 0.5, 100.0).expect("partition");
        assert_eq!(value.direct_visible_w_m2.to_bits(), 0.0_f64.to_bits());
        assert_eq!(value.direct_nir_w_m2.to_bits(), 0.0_f64.to_bits());
        assert_eq!(
            (value.diffuse_visible_w_m2 + value.diffuse_nir_w_m2).to_bits(),
            0.1_f64.to_bits()
        );
    }

    #[test]
    fn subnormal_positive_solar_cosine_rejects_nonfinite_components() {
        assert!(matches!(
            weiss_norman_partition(1.0, f64::from_bits(1), 100.0),
            Err(SnowFreeAtmosphericError::Domain {
                field: "weiss_norman_component",
                ..
            })
        ));
    }
}
