use openwepp_unit_boundary::{BoundaryError, FractionUnitInterval, TemperatureCelsius};

use crate::error::MeteorologyError;

const KELVIN_OFFSET: f64 = 273.15;
const MOLAR_MASS_WATER_KG_PER_MOL: f64 = 0.018_015_28;
const UNIVERSAL_GAS_CONSTANT_J_MOL_K: f64 = 8.314_41;
const SATURATION_WATER_REFERENCE_KPA: f64 = 0.611;
const SATURATION_ICE_REFERENCE_KPA: f64 = 0.611_15;

/// Vapor pressure in kilopascals.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct VaporPressureKilopascals(f64);

impl VaporPressureKilopascals {
    /// Construct a non-negative finite vapor pressure.
    ///
    /// # Errors
    ///
    /// Returns a typed error when `value_kpa` is non-finite or negative.
    pub fn try_new(value_kpa: f64) -> Result<Self, MeteorologyError> {
        validate_non_negative("vapor_pressure_kpa", value_kpa)?;
        Ok(Self(value_kpa))
    }

    /// Raw value in kilopascals.
    #[must_use]
    pub const fn as_kilopascals(self) -> f64 {
        self.0
    }
}

/// Vapor density in kilograms per cubic meter.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct VaporDensityKilogramsPerCubicMeter(f64);

impl VaporDensityKilogramsPerCubicMeter {
    /// Construct a non-negative finite vapor density.
    ///
    /// # Errors
    ///
    /// Returns a typed error when `value_kg_m3` is non-finite or negative.
    pub fn try_new(value_kg_m3: f64) -> Result<Self, MeteorologyError> {
        validate_non_negative("vapor_density_kg_m3", value_kg_m3)?;
        Ok(Self(value_kg_m3))
    }

    /// Raw value in kilograms per cubic meter.
    #[must_use]
    pub const fn as_kilograms_per_cubic_meter(self) -> f64 {
        self.0
    }
}

/// Latent heat in joules per kilogram.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct LatentHeatJoulesPerKilogram(f64);

impl LatentHeatJoulesPerKilogram {
    /// Construct a positive finite latent heat.
    ///
    /// # Errors
    ///
    /// Returns a typed error when `value_j_kg` is non-finite or not positive.
    pub fn try_new(value_j_kg: f64) -> Result<Self, MeteorologyError> {
        validate_positive("latent_heat_j_kg", value_j_kg)?;
        Ok(Self(value_j_kg))
    }

    /// Raw value in joules per kilogram.
    #[must_use]
    pub const fn as_joules_per_kilogram(self) -> f64 {
        self.0
    }
}

/// Molecular diffusivity in square meters per second.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DiffusivitySquareMetersPerSecond(f64);

impl DiffusivitySquareMetersPerSecond {
    /// Construct a positive finite diffusivity.
    ///
    /// # Errors
    ///
    /// Returns a typed error when `value_m2_s` is non-finite or not positive.
    pub fn try_new(value_m2_s: f64) -> Result<Self, MeteorologyError> {
        validate_positive("diffusivity_m2_s", value_m2_s)?;
        Ok(Self(value_m2_s))
    }

    /// Raw value in square meters per second.
    #[must_use]
    pub const fn as_square_meters_per_second(self) -> f64 {
        self.0
    }
}

/// Thermal conductivity in watts per meter per kelvin.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ThermalConductivityWattsPerMeterKelvin(f64);

impl ThermalConductivityWattsPerMeterKelvin {
    /// Construct a positive finite thermal conductivity.
    ///
    /// # Errors
    ///
    /// Returns a typed error when `value_w_m_k` is non-finite or not positive.
    pub fn try_new(value_w_m_k: f64) -> Result<Self, MeteorologyError> {
        validate_positive("thermal_conductivity_w_m_k", value_w_m_k)?;
        Ok(Self(value_w_m_k))
    }

    /// Raw value in watts per meter per kelvin.
    #[must_use]
    pub const fn as_watts_per_meter_kelvin(self) -> f64 {
        self.0
    }
}

/// Convert Celsius to Kelvin with an absolute-zero guard.
///
/// # Errors
///
/// Returns a typed error when the Celsius value is at or below absolute zero.
pub fn celsius_to_kelvin(temperature: TemperatureCelsius) -> Result<f64, MeteorologyError> {
    let value_c = temperature.as_celsius();
    if value_c <= -KELVIN_OFFSET {
        return Err(MeteorologyError::BelowAbsoluteZero {
            quantity: "temperature_c",
            value_c,
        });
    }
    Ok(value_c + KELVIN_OFFSET)
}

/// Saturation vapor pressure over liquid water using the Harder-Pomeroy
/// Appendix-A Dingman form (`kPa`).
///
/// # Errors
///
/// Returns a typed error when the computed pressure is non-finite or negative.
pub fn saturation_vapor_pressure_water_kpa(
    temperature: TemperatureCelsius,
) -> Result<VaporPressureKilopascals, MeteorologyError> {
    let value_c = temperature.as_celsius();
    let exponent = 17.3 * value_c / (237.3 + value_c);
    VaporPressureKilopascals::try_new(SATURATION_WATER_REFERENCE_KPA * exponent.exp())
}

/// Saturation vapor pressure over ice using a Magnus-type ice helper (`kPa`).
///
/// This helper supports auto-phase psychrometric checks. It is not the source
/// of production precipitation partition behavior in SNOWDENSITY-10.3.5a.
///
/// # Errors
///
/// Returns a typed error when the computed pressure is non-finite or negative.
pub fn saturation_vapor_pressure_ice_kpa(
    temperature: TemperatureCelsius,
) -> Result<VaporPressureKilopascals, MeteorologyError> {
    let value_c = temperature.as_celsius();
    let exponent = 22.452 * value_c / (272.55 + value_c);
    VaporPressureKilopascals::try_new(SATURATION_ICE_REFERENCE_KPA * exponent.exp())
}

/// Saturation vapor pressure over ice at/below freezing and water above.
///
/// # Errors
///
/// Returns a typed error when the selected saturation helper fails.
pub fn saturation_vapor_pressure_auto_kpa(
    temperature: TemperatureCelsius,
) -> Result<VaporPressureKilopascals, MeteorologyError> {
    if temperature.as_celsius() <= 0.0 {
        saturation_vapor_pressure_ice_kpa(temperature)
    } else {
        saturation_vapor_pressure_water_kpa(temperature)
    }
}

/// Actual vapor pressure from relative humidity and auto-phase saturation.
///
/// # Errors
///
/// Returns a typed error when the saturation helper or output wrapper fails.
pub fn actual_vapor_pressure_from_relative_humidity_kpa(
    temperature: TemperatureCelsius,
    relative_humidity: FractionUnitInterval,
) -> Result<VaporPressureKilopascals, MeteorologyError> {
    let saturation = saturation_vapor_pressure_auto_kpa(temperature)?;
    VaporPressureKilopascals::try_new(saturation.as_kilopascals() * relative_humidity.as_fraction())
}

/// Relative humidity from air temperature and dew point using the liquid-water
/// Harder-Pomeroy saturation helper.
///
/// # Errors
///
/// Returns a typed error when the computed ratio is outside `[0, 1]`.
pub fn relative_humidity_from_dew_point(
    air_temperature: TemperatureCelsius,
    dew_point: TemperatureCelsius,
) -> Result<FractionUnitInterval, MeteorologyError> {
    let actual = saturation_vapor_pressure_water_kpa(dew_point)?;
    let saturation = saturation_vapor_pressure_water_kpa(air_temperature)?;
    let ratio = actual.as_kilopascals() / saturation.as_kilopascals();
    Ok(FractionUnitInterval::try_new(ratio)?)
}

/// Dew point from air temperature and relative humidity using the inverse of
/// the Harder-Pomeroy liquid-water saturation helper.
///
/// # Errors
///
/// Returns a typed error when relative humidity is zero or the computed
/// dewpoint wrapper rejects the value.
pub fn dew_point_from_relative_humidity(
    air_temperature: TemperatureCelsius,
    relative_humidity: FractionUnitInterval,
) -> Result<TemperatureCelsius, MeteorologyError> {
    let relative_humidity = relative_humidity.as_fraction();
    if relative_humidity <= 0.0 {
        return Err(MeteorologyError::NonPositive {
            quantity: "relative_humidity",
            value: relative_humidity,
        });
    }
    let saturation = saturation_vapor_pressure_water_kpa(air_temperature)?;
    let actual_kpa = saturation.as_kilopascals() * relative_humidity;
    let log_ratio = (actual_kpa / SATURATION_WATER_REFERENCE_KPA).ln();
    Ok(TemperatureCelsius::try_new(
        237.3 * log_ratio / (17.3 - log_ratio),
    )?)
}

/// Vapor density from vapor pressure and temperature via the ideal gas law.
///
/// # Errors
///
/// Returns a typed error when temperature is at/below absolute zero or the
/// output wrapper rejects the density.
pub fn vapor_density_from_pressure_and_temperature(
    vapor_pressure: VaporPressureKilopascals,
    temperature: TemperatureCelsius,
) -> Result<VaporDensityKilogramsPerCubicMeter, MeteorologyError> {
    let temperature_k = celsius_to_kelvin(temperature)?;
    let pressure_pa = vapor_pressure.as_kilopascals() * 1_000.0;
    VaporDensityKilogramsPerCubicMeter::try_new(
        MOLAR_MASS_WATER_KG_PER_MOL * pressure_pa
            / (UNIVERSAL_GAS_CONSTANT_J_MOL_K * temperature_k),
    )
}

/// Molecular diffusivity of water vapor in air from Harder-Pomeroy Appendix A.
///
/// # Errors
///
/// Returns a typed error when temperature is at/below absolute zero or the
/// output wrapper rejects the diffusivity.
pub fn molecular_diffusivity_water_vapor_in_air(
    air_temperature: TemperatureCelsius,
) -> Result<DiffusivitySquareMetersPerSecond, MeteorologyError> {
    let temperature_k = celsius_to_kelvin(air_temperature)?;
    DiffusivitySquareMetersPerSecond::try_new(2.06e-5 * (temperature_k / KELVIN_OFFSET).powf(1.75))
}

/// Thermal conductivity of air from Harder-Pomeroy Appendix A.
///
/// # Errors
///
/// Returns a typed error when temperature is at/below absolute zero or the
/// output wrapper rejects the conductivity.
pub fn thermal_conductivity_air(
    air_temperature: TemperatureCelsius,
) -> Result<ThermalConductivityWattsPerMeterKelvin, MeteorologyError> {
    let temperature_k = celsius_to_kelvin(air_temperature)?;
    ThermalConductivityWattsPerMeterKelvin::try_new(0.000_063 * temperature_k + 0.006_73)
}

/// Latent heat of sublimation from Harder-Pomeroy Appendix A.
///
/// # Errors
///
/// Returns a typed error when the output wrapper rejects the value.
pub fn latent_heat_sublimation(
    temperature: TemperatureCelsius,
) -> Result<LatentHeatJoulesPerKilogram, MeteorologyError> {
    let value_c = temperature.as_celsius();
    LatentHeatJoulesPerKilogram::try_new(
        1_000.0 * (2_834.1 - 0.29 * value_c - 0.004 * value_c.powi(2)),
    )
}

/// Latent heat of vaporization from Harder-Pomeroy Appendix A.
///
/// # Errors
///
/// Returns a typed error when the output wrapper rejects the value.
pub fn latent_heat_vaporization(
    temperature: TemperatureCelsius,
) -> Result<LatentHeatJoulesPerKilogram, MeteorologyError> {
    LatentHeatJoulesPerKilogram::try_new(1_000.0 * (2_501.0 - 2.361 * temperature.as_celsius()))
}

/// Latent heat selected by hydrometeor temperature.
///
/// # Errors
///
/// Returns a typed error when the selected latent-heat helper fails.
pub fn latent_heat_for_hydrometeor_temperature(
    temperature: TemperatureCelsius,
) -> Result<LatentHeatJoulesPerKilogram, MeteorologyError> {
    if temperature.as_celsius() < 0.0 {
        latent_heat_sublimation(temperature)
    } else {
        latent_heat_vaporization(temperature)
    }
}

fn validate_non_negative(boundary: &'static str, value: f64) -> Result<(), MeteorologyError> {
    if !value.is_finite() {
        return Err(BoundaryError::NonFinite { boundary, value }.into());
    }
    if value < 0.0 {
        return Err(BoundaryError::BelowMinimum {
            boundary,
            value,
            minimum: 0.0,
        }
        .into());
    }
    Ok(())
}

fn validate_positive(boundary: &'static str, value: f64) -> Result<(), MeteorologyError> {
    if !value.is_finite() {
        return Err(BoundaryError::NonFinite { boundary, value }.into());
    }
    if value <= 0.0 {
        return Err(MeteorologyError::NonPositive {
            quantity: boundary,
            value,
        });
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn assert_close(actual: f64, expected: f64, tolerance: f64) {
        assert!(
            (actual - expected).abs() <= tolerance,
            "actual {actual} expected {expected} tolerance {tolerance}"
        );
    }

    #[test]
    fn saturation_vapor_pressure_reference_values_are_stable() {
        let zero = TemperatureCelsius::try_new(0.0).expect("valid temperature");
        let twenty = TemperatureCelsius::try_new(20.0).expect("valid temperature");
        let minus_ten = TemperatureCelsius::try_new(-10.0).expect("valid temperature");

        assert_close(
            saturation_vapor_pressure_water_kpa(zero)
                .expect("valid water saturation")
                .as_kilopascals(),
            0.611,
            1.0e-12,
        );
        assert_close(
            saturation_vapor_pressure_water_kpa(twenty)
                .expect("valid water saturation")
                .as_kilopascals(),
            2.344_507_723_843_366,
            1.0e-12,
        );
        assert_close(
            saturation_vapor_pressure_ice_kpa(minus_ten)
                .expect("valid ice saturation")
                .as_kilopascals(),
            0.259_872_474_628_026_2,
            1.0e-12,
        );
    }

    #[test]
    fn dewpoint_and_relative_humidity_round_trip() {
        let air = TemperatureCelsius::try_new(10.0).expect("valid air temperature");
        let rh = FractionUnitInterval::try_new(0.5).expect("valid relative humidity");
        let dewpoint =
            dew_point_from_relative_humidity(air, rh).expect("valid dewpoint calculation");
        assert_close(dewpoint.as_celsius(), 0.087_929_800_431_988_89, 1.0e-12);

        let round_trip =
            relative_humidity_from_dew_point(air, dewpoint).expect("valid RH calculation");
        assert_close(round_trip.as_fraction(), 0.5, 1.0e-12);
    }

    #[test]
    fn latent_heat_diffusivity_and_conductivity_reference_values_are_stable() {
        let zero = TemperatureCelsius::try_new(0.0).expect("valid temperature");
        let minus_ten = TemperatureCelsius::try_new(-10.0).expect("valid temperature");

        assert_close(
            latent_heat_vaporization(zero)
                .expect("valid vaporization")
                .as_joules_per_kilogram(),
            2_501_000.0,
            1.0e-9,
        );
        assert_close(
            latent_heat_sublimation(minus_ten)
                .expect("valid sublimation")
                .as_joules_per_kilogram(),
            2_836_600.0,
            1.0e-9,
        );
        assert_close(
            molecular_diffusivity_water_vapor_in_air(zero)
                .expect("valid diffusivity")
                .as_square_meters_per_second(),
            2.06e-5,
            1.0e-15,
        );
        assert_close(
            thermal_conductivity_air(zero)
                .expect("valid conductivity")
                .as_watts_per_meter_kelvin(),
            0.023_938_45,
            1.0e-12,
        );
    }

    #[test]
    fn absolute_zero_guard_rejects_nonphysical_temperature() {
        let too_cold = TemperatureCelsius::try_new(-273.15).expect("finite temperature");
        assert!(matches!(
            celsius_to_kelvin(too_cold),
            Err(MeteorologyError::BelowAbsoluteZero { .. })
        ));
    }
}
