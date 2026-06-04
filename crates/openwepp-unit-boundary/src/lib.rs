//! Unit-safe boundary wrappers for hydrologic state and flux surfaces.
//!
//! This crate defines typed boundary values with explicit constructors and
//! conversion helpers for high-risk scalar interfaces used at kernel and
//! orchestrator boundaries.

#![deny(unsafe_code)]
#![allow(clippy::module_name_repetitions)]

use std::error::Error;
use std::fmt;

const MILLIMETERS_PER_METER: f64 = 1_000.0;
const LITERS_PER_CUBIC_METER: f64 = 1_000.0;
const SECONDS_PER_HOUR: f64 = 3_600.0;

/// Boundary-construction or conversion error.
#[derive(Debug, Clone, PartialEq)]
pub enum BoundaryError {
    /// Value is NaN or infinite.
    NonFinite { boundary: &'static str, value: f64 },
    /// Value is below an allowed lower bound.
    BelowMinimum {
        boundary: &'static str,
        value: f64,
        minimum: f64,
    },
    /// Value is above an allowed upper bound.
    AboveMaximum {
        boundary: &'static str,
        value: f64,
        maximum: f64,
    },
}

impl fmt::Display for BoundaryError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NonFinite { boundary, value } => {
                write!(f, "{boundary} must be finite; received {value}")
            }
            Self::BelowMinimum {
                boundary,
                value,
                minimum,
            } => {
                write!(f, "{boundary} must be >= {minimum}; received {value}")
            }
            Self::AboveMaximum {
                boundary,
                value,
                maximum,
            } => {
                write!(f, "{boundary} must be <= {maximum}; received {value}")
            }
        }
    }
}

impl Error for BoundaryError {}

/// Drainage or contributing area at a boundary (`m^2`).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SurfaceAreaSquareMeters(f64);

impl SurfaceAreaSquareMeters {
    /// Construct an area value in square meters.
    ///
    /// Domain guards:
    /// - finite
    /// - strictly positive (`> 0.0`)
    ///
    /// # Errors
    ///
    /// Returns:
    /// - [`BoundaryError::NonFinite`] when `value` is `NaN` or infinite.
    /// - [`BoundaryError::BelowMinimum`] when `value <= 0.0`.
    pub fn try_new(value: f64) -> Result<Self, BoundaryError> {
        validate_finite("surface_area_m2", value)?;
        validate_minimum("surface_area_m2", value, f64::EPSILON)?;
        Ok(Self(value))
    }

    /// Raw value in `m^2`.
    #[must_use]
    pub const fn as_square_meters(self) -> f64 {
        self.0
    }
}

/// Runoff depth at a boundary (`mm`).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct RunoffDepthMillimeters(f64);

impl RunoffDepthMillimeters {
    /// Construct runoff depth in millimeters.
    ///
    /// Domain guards:
    /// - finite
    /// - non-negative (`>= 0.0`)
    ///
    /// # Errors
    ///
    /// Returns:
    /// - [`BoundaryError::NonFinite`] when `value_mm` is `NaN` or infinite.
    /// - [`BoundaryError::BelowMinimum`] when `value_mm < 0.0`.
    pub fn try_new(value_mm: f64) -> Result<Self, BoundaryError> {
        validate_non_negative("runoff_depth_mm", value_mm)?;
        Ok(Self(value_mm))
    }

    /// Construct runoff depth from meters.
    ///
    /// # Errors
    ///
    /// Returns:
    /// - [`BoundaryError::NonFinite`] when input or converted depth is non-finite.
    /// - [`BoundaryError::BelowMinimum`] when `value_m < 0.0`.
    pub fn from_meters(value_m: f64) -> Result<Self, BoundaryError> {
        validate_non_negative("runoff_depth_m", value_m)?;
        let value_mm = checked_mul("runoff_depth_mm", value_m, MILLIMETERS_PER_METER)?;
        Self::try_new(value_mm)
    }

    /// Raw value in millimeters.
    #[must_use]
    pub const fn as_millimeters(self) -> f64 {
        self.0
    }

    /// Value converted to meters.
    #[must_use]
    pub fn as_meters(self) -> f64 {
        self.0 / MILLIMETERS_PER_METER
    }

    /// Convert runoff depth to equivalent storage volume for a given area.
    ///
    /// # Errors
    ///
    /// Returns [`BoundaryError::NonFinite`] when multiplication overflows.
    pub fn to_volume(
        self,
        area_m2: SurfaceAreaSquareMeters,
    ) -> Result<StorageVolumeCubicMeters, BoundaryError> {
        let depth_m = self.as_meters();
        let volume = checked_mul("storage_volume_m3", depth_m, area_m2.as_square_meters())?;
        StorageVolumeCubicMeters::try_new(volume)
    }
}

/// Flow rate at a boundary (`m^3/s`).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct FlowRateCubicMetersPerSecond(f64);

impl FlowRateCubicMetersPerSecond {
    /// Construct flow rate in cubic meters per second.
    ///
    /// Domain guards:
    /// - finite
    /// - non-negative (`>= 0.0`)
    ///
    /// # Errors
    ///
    /// Returns:
    /// - [`BoundaryError::NonFinite`] when `value_m3_per_s` is `NaN` or infinite.
    /// - [`BoundaryError::BelowMinimum`] when `value_m3_per_s < 0.0`.
    pub fn try_new(value_m3_per_s: f64) -> Result<Self, BoundaryError> {
        validate_non_negative("flow_rate_m3_s", value_m3_per_s)?;
        Ok(Self(value_m3_per_s))
    }

    /// Construct flow rate from liters per second.
    ///
    /// # Errors
    ///
    /// Returns:
    /// - [`BoundaryError::NonFinite`] when `value_l_per_s` is `NaN` or infinite.
    /// - [`BoundaryError::BelowMinimum`] when `value_l_per_s < 0.0`.
    pub fn from_liters_per_second(value_l_per_s: f64) -> Result<Self, BoundaryError> {
        validate_non_negative("flow_rate_l_s", value_l_per_s)?;
        let value_m3_per_s = value_l_per_s / LITERS_PER_CUBIC_METER;
        Self::try_new(value_m3_per_s)
    }

    /// Raw value in cubic meters per second.
    #[must_use]
    pub const fn as_cubic_meters_per_second(self) -> f64 {
        self.0
    }

    /// Value converted to liters per second.
    #[must_use]
    pub fn as_liters_per_second(self) -> f64 {
        self.0 * LITERS_PER_CUBIC_METER
    }
}

/// Storage volume at a boundary (`m^3`).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct StorageVolumeCubicMeters(f64);

impl StorageVolumeCubicMeters {
    /// Construct storage volume in cubic meters.
    ///
    /// Domain guards:
    /// - finite
    /// - non-negative (`>= 0.0`)
    ///
    /// # Errors
    ///
    /// Returns:
    /// - [`BoundaryError::NonFinite`] when `value_m3` is `NaN` or infinite.
    /// - [`BoundaryError::BelowMinimum`] when `value_m3 < 0.0`.
    pub fn try_new(value_m3: f64) -> Result<Self, BoundaryError> {
        validate_non_negative("storage_volume_m3", value_m3)?;
        Ok(Self(value_m3))
    }

    /// Construct storage volume from liters.
    ///
    /// # Errors
    ///
    /// Returns:
    /// - [`BoundaryError::NonFinite`] when `value_liters` is `NaN` or infinite.
    /// - [`BoundaryError::BelowMinimum`] when `value_liters < 0.0`.
    pub fn from_liters(value_liters: f64) -> Result<Self, BoundaryError> {
        validate_non_negative("storage_volume_l", value_liters)?;
        let value_m3 = value_liters / LITERS_PER_CUBIC_METER;
        Self::try_new(value_m3)
    }

    /// Raw value in cubic meters.
    #[must_use]
    pub const fn as_cubic_meters(self) -> f64 {
        self.0
    }

    /// Value converted to liters.
    #[must_use]
    pub fn as_liters(self) -> f64 {
        self.0 * LITERS_PER_CUBIC_METER
    }

    /// Convert storage volume to equivalent depth for a given area.
    ///
    /// # Errors
    ///
    /// Returns:
    /// - [`BoundaryError::BelowMinimum`] when `area_m2 <= 0.0`.
    /// - [`BoundaryError::NonFinite`] when division or conversion produces a non-finite value.
    pub fn to_depth(
        self,
        area_m2: SurfaceAreaSquareMeters,
    ) -> Result<RunoffDepthMillimeters, BoundaryError> {
        let depth_m = checked_div("runoff_depth_m", self.0, area_m2.as_square_meters())?;
        RunoffDepthMillimeters::from_meters(depth_m)
    }
}

/// Process rate at a boundary (`mm/hr`).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ProcessRateMillimetersPerHour(f64);

impl ProcessRateMillimetersPerHour {
    /// Construct a process rate in millimeters per hour.
    ///
    /// Domain guards:
    /// - finite
    /// - non-negative (`>= 0.0`)
    ///
    /// # Errors
    ///
    /// Returns:
    /// - [`BoundaryError::NonFinite`] when `value_mm_per_hr` is `NaN` or infinite.
    /// - [`BoundaryError::BelowMinimum`] when `value_mm_per_hr < 0.0`.
    pub fn try_new(value_mm_per_hr: f64) -> Result<Self, BoundaryError> {
        validate_non_negative("process_rate_mm_hr", value_mm_per_hr)?;
        Ok(Self(value_mm_per_hr))
    }

    /// Construct a process rate from meters per second.
    ///
    /// # Errors
    ///
    /// Returns:
    /// - [`BoundaryError::NonFinite`] when input or conversion intermediate is non-finite.
    /// - [`BoundaryError::BelowMinimum`] when `value_m_per_s < 0.0`.
    pub fn from_meters_per_second(value_m_per_s: f64) -> Result<Self, BoundaryError> {
        validate_non_negative("process_rate_m_s", value_m_per_s)?;
        let mm_per_s = checked_mul("process_rate_mm_s", value_m_per_s, MILLIMETERS_PER_METER)?;
        let mm_per_hr = checked_mul("process_rate_mm_hr", mm_per_s, SECONDS_PER_HOUR)?;
        Self::try_new(mm_per_hr)
    }

    /// Raw value in millimeters per hour.
    #[must_use]
    pub const fn as_millimeters_per_hour(self) -> f64 {
        self.0
    }

    /// Value converted to meters per second.
    #[must_use]
    pub fn as_meters_per_second(self) -> f64 {
        self.0 / MILLIMETERS_PER_METER / SECONDS_PER_HOUR
    }
}

/// Water depth at a runtime seam (`m`).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct WaterDepthMeters(f64);

impl WaterDepthMeters {
    /// Construct a water depth in meters.
    ///
    /// Domain guards:
    /// - finite
    /// - non-negative (`>= 0.0`)
    ///
    /// # Errors
    ///
    /// Returns:
    /// - [`BoundaryError::NonFinite`] when `value_m` is `NaN` or infinite.
    /// - [`BoundaryError::BelowMinimum`] when `value_m < 0.0`.
    pub fn try_new(value_m: f64) -> Result<Self, BoundaryError> {
        validate_non_negative("water_depth_m", value_m)?;
        Ok(Self(value_m))
    }

    /// Raw value in meters.
    #[must_use]
    pub const fn as_meters(self) -> f64 {
        self.0
    }
}

/// Elapsed time at a runtime seam (`s`).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ElapsedTimeSeconds(f64);

impl ElapsedTimeSeconds {
    /// Construct an elapsed time in seconds.
    ///
    /// Domain guards:
    /// - finite
    /// - non-negative (`>= 0.0`)
    ///
    /// # Errors
    ///
    /// Returns:
    /// - [`BoundaryError::NonFinite`] when `value_s` is `NaN` or infinite.
    /// - [`BoundaryError::BelowMinimum`] when `value_s < 0.0`.
    pub fn try_new(value_s: f64) -> Result<Self, BoundaryError> {
        validate_non_negative("elapsed_time_s", value_s)?;
        Ok(Self(value_s))
    }

    /// Raw value in seconds.
    #[must_use]
    pub const fn as_seconds(self) -> f64 {
        self.0
    }
}

/// Hour-of-day marker at a runtime seam (`h`).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct HourOfDay(f64);

impl HourOfDay {
    /// Construct an hour-of-day marker.
    ///
    /// Domain guards:
    /// - finite
    /// - within `[0.0, 24.0]`
    ///
    /// # Errors
    ///
    /// Returns:
    /// - [`BoundaryError::NonFinite`] when `value_h` is `NaN` or infinite.
    /// - [`BoundaryError::BelowMinimum`] when `value_h < 0.0`.
    /// - [`BoundaryError::AboveMaximum`] when `value_h > 24.0`.
    pub fn try_new(value_h: f64) -> Result<Self, BoundaryError> {
        validate_finite("hour_of_day_h", value_h)?;
        validate_minimum("hour_of_day_h", value_h, 0.0)?;
        validate_maximum("hour_of_day_h", value_h, 24.0)?;
        Ok(Self(value_h))
    }

    /// Raw value in hours.
    #[must_use]
    pub const fn as_hours(self) -> f64 {
        self.0
    }
}

/// Linear rate at a runtime seam (`m s^-1`).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct LinearRateMetersPerSecond(f64);

impl LinearRateMetersPerSecond {
    /// Construct a linear rate in meters per second.
    ///
    /// Domain guards:
    /// - finite
    /// - non-negative (`>= 0.0`)
    ///
    /// # Errors
    ///
    /// Returns:
    /// - [`BoundaryError::NonFinite`] when `value_m_per_s` is `NaN` or infinite.
    /// - [`BoundaryError::BelowMinimum`] when `value_m_per_s < 0.0`.
    pub fn try_new(value_m_per_s: f64) -> Result<Self, BoundaryError> {
        validate_non_negative("linear_rate_m_s", value_m_per_s)?;
        Ok(Self(value_m_per_s))
    }

    /// Raw value in meters per second.
    #[must_use]
    pub const fn as_meters_per_second(self) -> f64 {
        self.0
    }
}

/// Daily solar radiation at the climate runtime seam (`Ly d^-1`).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SolarRadiationLangleysPerDay(f64);

impl SolarRadiationLangleysPerDay {
    /// Construct daily radiation in Langleys per day.
    ///
    /// Domain guards:
    /// - finite
    /// - non-negative (`>= 0.0`)
    ///
    /// # Errors
    ///
    /// Returns:
    /// - [`BoundaryError::NonFinite`] when `value_ly_per_day` is `NaN` or infinite.
    /// - [`BoundaryError::BelowMinimum`] when `value_ly_per_day < 0.0`.
    pub fn try_new(value_ly_per_day: f64) -> Result<Self, BoundaryError> {
        validate_non_negative("solar_radiation_ly_d", value_ly_per_day)?;
        Ok(Self(value_ly_per_day))
    }

    /// Raw value in Langleys per day.
    #[must_use]
    pub const fn as_langleys_per_day(self) -> f64 {
        self.0
    }
}

/// Daily solar radiation (`MJ m^-2 d^-1`).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SolarRadiationMegajoulesPerSquareMeterPerDay(f64);

impl SolarRadiationMegajoulesPerSquareMeterPerDay {
    /// Construct daily radiation in megajoules per square meter per day.
    ///
    /// Domain guards:
    /// - finite
    /// - non-negative (`>= 0.0`)
    ///
    /// # Errors
    ///
    /// Returns:
    /// - [`BoundaryError::NonFinite`] when `value_mj_m2_day` is `NaN` or infinite.
    /// - [`BoundaryError::BelowMinimum`] when `value_mj_m2_day < 0.0`.
    pub fn try_new(value_mj_m2_day: f64) -> Result<Self, BoundaryError> {
        validate_non_negative("solar_radiation_mj_m2_d", value_mj_m2_day)?;
        Ok(Self(value_mj_m2_day))
    }

    /// Raw value in megajoules per square meter per day.
    #[must_use]
    pub const fn as_megajoules_per_square_meter_per_day(self) -> f64 {
        self.0
    }
}

/// Hourly solar radiation (`MJ m^-2 h^-1`).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SolarRadiationMegajoulesPerSquareMeterPerHour(f64);

impl SolarRadiationMegajoulesPerSquareMeterPerHour {
    /// Construct hourly radiation in megajoules per square meter per hour.
    ///
    /// Domain guards:
    /// - finite
    /// - non-negative (`>= 0.0`)
    ///
    /// # Errors
    ///
    /// Returns:
    /// - [`BoundaryError::NonFinite`] when `value_mj_m2_hour` is `NaN` or infinite.
    /// - [`BoundaryError::BelowMinimum`] when `value_mj_m2_hour < 0.0`.
    pub fn try_new(value_mj_m2_hour: f64) -> Result<Self, BoundaryError> {
        validate_non_negative("solar_radiation_mj_m2_h", value_mj_m2_hour)?;
        Ok(Self(value_mj_m2_hour))
    }

    /// Raw value in megajoules per square meter per hour.
    #[must_use]
    pub const fn as_megajoules_per_square_meter_per_hour(self) -> f64 {
        self.0
    }
}

/// Temperature at a runtime seam (`degC`).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct TemperatureCelsius(f64);

impl TemperatureCelsius {
    /// Construct a Celsius temperature.
    ///
    /// Domain guards:
    /// - finite
    ///
    /// # Errors
    ///
    /// Returns [`BoundaryError::NonFinite`] when `value_c` is `NaN` or infinite.
    pub fn try_new(value_c: f64) -> Result<Self, BoundaryError> {
        validate_finite("temperature_c", value_c)?;
        Ok(Self(value_c))
    }

    /// Raw value in degrees Celsius.
    #[must_use]
    pub const fn as_celsius(self) -> f64 {
        self.0
    }
}

/// Density at a runtime seam (`kg m^-3`).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DensityKilogramsPerCubicMeter(f64);

impl DensityKilogramsPerCubicMeter {
    /// Construct density in kilograms per cubic meter.
    ///
    /// Domain guards:
    /// - finite
    /// - non-negative (`>= 0.0`)
    ///
    /// # Errors
    ///
    /// Returns:
    /// - [`BoundaryError::NonFinite`] when `value_kg_m3` is `NaN` or infinite.
    /// - [`BoundaryError::BelowMinimum`] when `value_kg_m3 < 0.0`.
    pub fn try_new(value_kg_m3: f64) -> Result<Self, BoundaryError> {
        validate_non_negative("density_kg_m3", value_kg_m3)?;
        Ok(Self(value_kg_m3))
    }

    /// Raw value in kilograms per cubic meter.
    #[must_use]
    pub const fn as_kilograms_per_cubic_meter(self) -> f64 {
        self.0
    }
}

/// Dimensionless fraction constrained to `[0, 1]`.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct FractionUnitInterval(f64);

impl FractionUnitInterval {
    /// Construct a unit-interval fraction.
    ///
    /// Domain guards:
    /// - finite
    /// - within `[0.0, 1.0]`
    ///
    /// # Errors
    ///
    /// Returns:
    /// - [`BoundaryError::NonFinite`] when `value` is `NaN` or infinite.
    /// - [`BoundaryError::BelowMinimum`] when `value < 0.0`.
    /// - [`BoundaryError::AboveMaximum`] when `value > 1.0`.
    pub fn try_new(value: f64) -> Result<Self, BoundaryError> {
        validate_finite("fraction_unit_interval", value)?;
        validate_minimum("fraction_unit_interval", value, 0.0)?;
        validate_maximum("fraction_unit_interval", value, 1.0)?;
        Ok(Self(value))
    }

    /// Raw dimensionless fraction.
    #[must_use]
    pub const fn as_fraction(self) -> f64 {
        self.0
    }
}

fn validate_finite(boundary: &'static str, value: f64) -> Result<(), BoundaryError> {
    if value.is_finite() {
        Ok(())
    } else {
        Err(BoundaryError::NonFinite { boundary, value })
    }
}

fn validate_minimum(boundary: &'static str, value: f64, minimum: f64) -> Result<(), BoundaryError> {
    if value >= minimum {
        Ok(())
    } else {
        Err(BoundaryError::BelowMinimum {
            boundary,
            value,
            minimum,
        })
    }
}

fn validate_maximum(boundary: &'static str, value: f64, maximum: f64) -> Result<(), BoundaryError> {
    if value <= maximum {
        Ok(())
    } else {
        Err(BoundaryError::AboveMaximum {
            boundary,
            value,
            maximum,
        })
    }
}

fn validate_non_negative(boundary: &'static str, value: f64) -> Result<(), BoundaryError> {
    validate_finite(boundary, value)?;
    validate_minimum(boundary, value, 0.0)
}

fn checked_mul(boundary: &'static str, lhs: f64, rhs: f64) -> Result<f64, BoundaryError> {
    let result = lhs * rhs;
    validate_finite(boundary, result)?;
    Ok(result)
}

fn checked_div(
    boundary: &'static str,
    numerator: f64,
    denominator: f64,
) -> Result<f64, BoundaryError> {
    validate_finite(boundary, denominator)?;
    validate_minimum(boundary, denominator, f64::EPSILON)?;
    let result = numerator / denominator;
    validate_finite(boundary, result)?;
    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn assert_close(left: f64, right: f64) {
        assert!((left - right).abs() < 1.0e-12, "left={left}, right={right}");
    }

    #[test]
    fn runoff_depth_rejects_negative() {
        let error =
            RunoffDepthMillimeters::try_new(-0.001).expect_err("negative runoff depth must fail");
        assert_eq!(
            error,
            BoundaryError::BelowMinimum {
                boundary: "runoff_depth_mm",
                value: -0.001,
                minimum: 0.0,
            }
        );
    }

    #[test]
    fn runoff_depth_rejects_non_finite() {
        let error =
            RunoffDepthMillimeters::try_new(f64::NAN).expect_err("nan runoff depth must fail");
        match error {
            BoundaryError::NonFinite { boundary, value } => {
                assert_eq!(boundary, "runoff_depth_mm");
                assert!(value.is_nan(), "expected NaN, received {value}");
            }
            other => {
                panic!("unexpected error variant: {other:?}");
            }
        }
    }

    #[test]
    fn runoff_depth_meters_round_trip() {
        let runoff = RunoffDepthMillimeters::from_meters(0.025).expect("valid meters");
        assert_close(runoff.as_millimeters(), 25.0);
        assert_close(runoff.as_meters(), 0.025);
    }

    #[test]
    fn runoff_depth_to_volume_conversion() {
        let runoff = RunoffDepthMillimeters::try_new(12.0).expect("valid runoff");
        let area = SurfaceAreaSquareMeters::try_new(5_000.0).expect("valid area");
        let volume = runoff.to_volume(area).expect("convertible");
        assert_close(volume.as_cubic_meters(), 60.0);
    }

    #[test]
    fn flow_rate_liters_round_trip() {
        let flow =
            FlowRateCubicMetersPerSecond::from_liters_per_second(250.0).expect("valid liters/s");
        assert_close(flow.as_cubic_meters_per_second(), 0.25);
        assert_close(flow.as_liters_per_second(), 250.0);
    }

    #[test]
    fn storage_volume_to_depth_conversion() {
        let storage = StorageVolumeCubicMeters::try_new(60.0).expect("valid volume");
        let area = SurfaceAreaSquareMeters::try_new(5_000.0).expect("valid area");
        let depth = storage.to_depth(area).expect("convertible");
        assert_close(depth.as_millimeters(), 12.0);
    }

    #[test]
    fn storage_volume_to_depth_rejects_zero_area() {
        let error = SurfaceAreaSquareMeters::try_new(0.0).expect_err("zero area must fail");
        assert_eq!(
            error,
            BoundaryError::BelowMinimum {
                boundary: "surface_area_m2",
                value: 0.0,
                minimum: f64::EPSILON,
            }
        );
    }

    #[test]
    fn process_rate_meters_per_second_round_trip() {
        let rate =
            ProcessRateMillimetersPerHour::from_meters_per_second(2.5e-6).expect("valid rate");
        assert_close(rate.as_millimeters_per_hour(), 9.0);
        assert_close(rate.as_meters_per_second(), 2.5e-6);
    }

    #[test]
    fn process_rate_rejects_non_finite_conversion() {
        let error = ProcessRateMillimetersPerHour::from_meters_per_second(f64::INFINITY)
            .expect_err("infinite rate must fail");
        assert_eq!(
            error,
            BoundaryError::NonFinite {
                boundary: "process_rate_m_s",
                value: f64::INFINITY,
            }
        );
    }

    #[test]
    fn runoff_conversion_rejects_overflow() {
        let error =
            RunoffDepthMillimeters::from_meters(f64::MAX).expect_err("overflow to mm must fail");
        assert_eq!(
            error,
            BoundaryError::NonFinite {
                boundary: "runoff_depth_mm",
                value: f64::INFINITY,
            }
        );
    }

    #[test]
    fn water_depth_meters_rejects_negative() {
        let error = WaterDepthMeters::try_new(-0.001).expect_err("negative depth must fail");
        assert_eq!(
            error,
            BoundaryError::BelowMinimum {
                boundary: "water_depth_m",
                value: -0.001,
                minimum: 0.0,
            }
        );
    }

    #[test]
    fn elapsed_time_seconds_preserves_value() {
        let elapsed = ElapsedTimeSeconds::try_new(7_200.0).expect("valid elapsed time");
        assert_close(elapsed.as_seconds(), 7_200.0);
    }

    #[test]
    fn hour_of_day_rejects_out_of_range() {
        let error = HourOfDay::try_new(24.5).expect_err("out-of-range hour must fail");
        assert_eq!(
            error,
            BoundaryError::AboveMaximum {
                boundary: "hour_of_day_h",
                value: 24.5,
                maximum: 24.0,
            }
        );
    }

    #[test]
    fn linear_rate_meters_per_second_preserves_value() {
        let rate = LinearRateMetersPerSecond::try_new(1.25e-6).expect("valid linear rate");
        assert_close(rate.as_meters_per_second(), 1.25e-6);
    }

    #[test]
    fn radiation_wrappers_reject_negative_values() {
        assert!(matches!(
            SolarRadiationLangleysPerDay::try_new(-1.0),
            Err(BoundaryError::BelowMinimum {
                boundary: "solar_radiation_ly_d",
                ..
            })
        ));
        assert!(matches!(
            SolarRadiationMegajoulesPerSquareMeterPerHour::try_new(-1.0),
            Err(BoundaryError::BelowMinimum {
                boundary: "solar_radiation_mj_m2_h",
                ..
            })
        ));
    }

    #[test]
    fn daily_mj_radiation_preserves_value() {
        let radiation = SolarRadiationMegajoulesPerSquareMeterPerDay::try_new(8.368)
            .expect("valid daily radiation");
        assert_close(radiation.as_megajoules_per_square_meter_per_day(), 8.368);
    }

    #[test]
    fn temperature_celsius_accepts_signed_finite_values() {
        let temperature = TemperatureCelsius::try_new(-17.5).expect("valid signed temperature");
        assert_close(temperature.as_celsius(), -17.5);
    }

    #[test]
    fn density_rejects_non_finite_values() {
        let error = DensityKilogramsPerCubicMeter::try_new(f64::INFINITY)
            .expect_err("infinite density must fail");
        assert_eq!(
            error,
            BoundaryError::NonFinite {
                boundary: "density_kg_m3",
                value: f64::INFINITY,
            }
        );
    }

    #[test]
    fn fraction_unit_interval_rejects_above_one() {
        let error =
            FractionUnitInterval::try_new(1.001).expect_err("above-unit fraction must fail");
        assert_eq!(
            error,
            BoundaryError::AboveMaximum {
                boundary: "fraction_unit_interval",
                value: 1.001,
                maximum: 1.0,
            }
        );
    }
}
