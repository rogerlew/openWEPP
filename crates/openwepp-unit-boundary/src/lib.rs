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
            other @ BoundaryError::BelowMinimum { .. } => {
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
}
